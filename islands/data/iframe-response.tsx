import { PageProps } from "$fresh/server.ts";
import { Signal } from "https://esm.sh/@preact/signals@1.2.2/dist/signals.js";
import { useEffect, useState } from "preact/hooks";

export default function IFRAMEResponse(props: {
  page: PageProps;
  signal: Signal<"Inactive" | "Loading" | "Error" | "Success">
}) {

  const [preventDoubleRun, setPreventDoubleRun] = useState(0);
  const [downloadLink, setDownloadLink] = useState<string>();
  const [error, setError] = useState<[number, string]>();

  useEffect(() => {

    if(preventDoubleRun !== 0) {
      return
    }
    setPreventDoubleRun(1)

    props.signal.value = "Loading"

    const xhr = new XMLHttpRequest()
    xhr.open("POST", "http://dtb-kampfrichter.de/pocket-paper-api/" + props.page.params["path"], true)
    xhr.setRequestHeader("Content-Type", "application/x-www-form-urlencoded")

    xhr.onload = (_ctx) => {
      if(xhr.readyState == xhr.DONE && xhr.status === 200) {
        setDownloadLink(xhr.response)
        props.signal.value = "Success"
      }
    }

    xhr.onerror = (_ctx) => {
      setError([xhr.status, xhr.response])
      props.signal.value = "Error"
    }

    xhr.onreadystatechange = function (_ctx) {
      if (xhr.readyState === 4 && xhr.status !== 200) {
        setError([xhr.status, xhr.response])
        props.signal.value = "Error"
      }
    };

    xhr.send(new URLSearchParams(props.page.data).toString())

  })

  function emitFinishEvent() {
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
            <button class="styled-button" download={downloadLink} onClick={emitFinishEvent}>Herunterladen</button>
            </>
          )
        case "Error":
          return(
            <>
            <h1 style={{ marginInline: "20px", textAlign: "center" }}>Ein Fehler ist aufgetreten!<br />Error {error?.[0]}</h1>
            <p>{error?.[1]}</p>
            <button class="styled-button" download={downloadLink} onClick={emitFinishEvent}>Schließen</button>
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
