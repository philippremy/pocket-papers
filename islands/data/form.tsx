import { useEffect } from "preact/hooks";
import { Signal } from "https://esm.sh/@preact/signals@1.2.2/dist/signals.d.ts";
import { APICallStatus } from "../../utils/form-state.tsx";

export default function FormComponent<T>(props: {
    method: "POST" | "GET",
    name: string,
    baseRoute: string
    signal: Signal<"Geradeturnen" | "Spiraleturnen" | "Sprung">
    target: string
}) {

    useEffect(() => {
        switch (props.signal.value) {
            case "Geradeturnen":
                (document.getElementsByName(props.name)[0] as HTMLFormElement).action = props.baseRoute + "straightline-pdf"
                break
            case "Spiraleturnen":
                (document.getElementsByName(props.name)[0] as HTMLFormElement).action = props.baseRoute + "spiral-pdf"
                break
            case "Sprung":
                (document.getElementsByName(props.name)[0] as HTMLFormElement).action = props.baseRoute + "vault-pdf"
                break
        }
    }, [props.signal.value])

    useEffect(() => {
        switch (props.signal.value) {
            case "Geradeturnen":
                (document.getElementsByName(props.name)[0] as HTMLFormElement).action = props.baseRoute + "straightline-pdf"
                break
            case "Spiraleturnen":
                (document.getElementsByName(props.name)[0] as HTMLFormElement).action = props.baseRoute + "spiral-pdf"
                break
            case "Sprung":
                (document.getElementsByName(props.name)[0] as HTMLFormElement).action = props.baseRoute + "vault-pdf"
                break
        }
    })

    return(
        <form id={props.name} name={props.name} action={props.baseRoute} method={props.method} target={props.target} />
    )
}

export function FormSubmit(props: {
    apiCallStatusSignal: Signal<APICallStatus>
    apiCallResponseSignal: Signal<{success: boolean, body: string} | null>
    form: string
}) {

    async function handleDownload(uuid: string) {
      
        const response = await fetch("https://api.dtb-kampfrichter.de/dyn-pocket-paper-download?" + new URLSearchParams({
            uuid: uuid
          }).toString(), {
            method: "GET",
            headers: {
              'Content-Type': 'application/json',
              'Accept': 'application/pdf'
            },
            mode: "cors",
          })
          if (!response.ok) {
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

    }

    async function handleSubmit() {
        if(!document.getElementsByTagName("form").namedItem(props.form)!.checkValidity()) {
            return
        }
        props.apiCallStatusSignal.value = APICallStatus.Processing
        const form = document.getElementsByTagName("form").namedItem(props.form)!
        const formData = new URLSearchParams();
        for(const elem of new FormData(form)) {
            formData.set(elem[0], elem[1].toString())
        }

        try {
            const response = await fetch(form.action, {
              method: form.method,
              headers: {
                'Content-Type': 'application/x-www-form-urlencoded',
              },
              body: formData,
            })

            if(!response.ok) {
                props.apiCallStatusSignal.value = APICallStatus.Error

            }
            props.apiCallStatusSignal.value = APICallStatus.Ok
            await handleDownload(await response.text())

        } catch (error) {
            props.apiCallStatusSignal.value = APICallStatus.Error
            props.apiCallResponseSignal.value = Object.assign({}, { success: false, body: error as string })
        }
    }

    function handleSave() {
        if(!document.getElementsByTagName("form").namedItem(props.form)!.checkValidity()) {
            return
        }
        const form = document.getElementsByTagName("form").namedItem(props.form)!
        const data = new FormData(form)
        const json = JSON.stringify(Object.fromEntries(data));
        const blob = new Blob([json], { type: 'application/octet-stream' });

        // Create a download link
        const a = document.createElement('a');
        a.href = URL.createObjectURL(blob);
        a.download = 'Hosentaschenkarte.pocketpaper';
        document.body.appendChild(a);
        a.click();
        document.body.removeChild(a);
    }

    return(
        <>
        <div class="styled-button-container">
            <button type="submit" class="styled-button" form={props.form} onClick={handleSubmit} disabled={props.apiCallStatusSignal.value === APICallStatus.Processing}>PDF erstellen</button>
            <button type="submit" class="styled-button" onClick={handleSave}>Hosentaschenkarte speichern</button>
        </div>
        </>
    )
    
}