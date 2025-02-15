import { useEffect } from "preact/hooks";
import { Signal } from "https://esm.sh/@preact/signals@1.2.2/dist/signals.d.ts";

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
    iframeSignal: Signal<"Inactive" | "Loading" | "Error" | "Success">
    form: string
}) {

    function handleSubmit() {
        if(!document.getElementsByTagName("form").namedItem(props.form)!.checkValidity()) {
            return
        }
        props.iframeSignal.value = "Loading";
    }

    return(
        <>
        <div class="styled-button-container">
            <button class="styled-button" form={props.form} onClick={handleSubmit}>PDF erstellen</button>
            <button class="styled-button" form={props.form} onClick={handleSubmit}>Hosentaschenkarte speichern</button>
        </div>
        </>
    )
    
}

export function FormResponse(props: {
    iframeSignal: Signal<"Inactive" | "Loading" | "Error" | "Success">
    name: string
}) {

    useEffect(() => {

        globalThis.addEventListener("message", (ev) => {
            if(ev.data === "iframeWorkFinished") {
                props.iframeSignal.value = "Inactive"
            }
        })

    })

    return(
        <>
        { props.iframeSignal.value === "Inactive" ?
        <>
        <iframe class="response-iframe" allowTransparency src={"/empty-iframe.html"} name={props.name} id={props.name} />
        </>
        :
        <>
        <iframe class="response-iframe active" allowTransparency src="/iframe-response" name={props.name} id={props.name} />
        </>
        }
        </>
    )

}