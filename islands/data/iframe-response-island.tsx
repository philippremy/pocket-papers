import { PageProps } from "$fresh/server.ts";
import { Signal } from "https://esm.sh/@preact/signals@1.2.2/dist/signals.js";
import { useEffect, useState } from "preact/hooks";

export default function IFRAMEResponse(props: {
  page: PageProps;
  signal: Signal<"Inactive" | "Loading" | "Error" | "Success">
}) {

  const [preventDoubleRun, setPreventDoubleRun] = useState(0);
  const [downloadLink, setDownloadLink] = useState<string>();
  const [error, setError] = useState<string>();

  useEffect(() => {

    if(preventDoubleRun !== 0) {
      return
    }
    setPreventDoubleRun(1)

    props.signal.value = "Loading"

    globalThis.top!.addEventListener("message", (ctx) => {
      const data = (ctx.data as { kind: string, data: { isError: boolean, text: string } })
      if(data.kind === "pocket-paper-request-responded") {
        if(data.data.isError) {
          setError(data.data.text)
          props.signal.value = "Error"
        } else {
          setDownloadLink(data.data.text)
          props.signal.value = "Success"
        }
      }
    })
  })

  async function emitFinishEvent() {

    const response = await fetch("https://api.dtb-kampfrichter.de/dyn-pocket-paper-download?" + new URLSearchParams({
      uuid: downloadLink!
    }).toString(), {
      method: "GET",
      headers: {
        'Content-Type': 'application/json',
        'Accept': 'application/pdf'
      },
      mode: "cors",
    })
    if (!response.ok) {
      setError("Ein Fehler beim Herunterladen ist aufgetreten.\n" + response.statusText + "\n" + await response.text())
      props.signal.value = "Error"
      return
    }
    // Extract the Content-Disposition header
    const contentDisposition = response.headers.get('Content-Disposition');
    let filename = 'Hosentaschenkarte.pdf'; // Default filename

    // Check if the Content-Disposition header contains the filename
    if (contentDisposition && contentDisposition.indexOf('attachment') !== -1) {
      const matches = contentDisposition.match(/filename="(.+)"/);
      if (matches && matches[1]) {
        filename = matches[1];
      }
    }

    const link = document.createElement('a'); // Create a download link
    link.href = URL.createObjectURL(await response.blob()); // Create an object URL for the Blob
    link.download = filename; // Set the filename for the download
    link.click(); // Trigger the download

    globalThis.top!.postMessage("iframeWorkFinished")

  }

  function draw() {
      switch (props.signal.value) {
        case "Loading":
          return(
            <>
            <h1>Die PDF wird generiert...</h1>
            <div class="loading-bar">
              <div class="inner-loading-bar" content="" />
            </div>
            </>
          )
        case "Success":
          return(
            <>
            <h1 style={{ marginInline: "20px", textAlign: "center" }}>Die Hosentaschenkarte ist fertig!</h1>
            <button class="styled-button" onClick={emitFinishEvent}>Herunterladen</button>
            </>
          )
        case "Error":
          return(
            <>
            <h1 style={{ marginInline: "20px", textAlign: "center" }}>Ein Fehler ist aufgetreten!<br />Error {error?.[0]}</h1>
            <p>{error?.[1]}</p>
            <button class="styled-button" onClick={emitFinishEvent}>Schließen</button>
            </>
          )
        default:
          return(
            <h1>Nichts zu tun. Bitte Seite neu laden.</h1>
          ) 
      }
  }

  return (
    <>
    {draw()}
    </>
  );
}
