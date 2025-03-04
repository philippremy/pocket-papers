import { PageProps } from "$fresh/server.ts";
import { Signal } from "@preact/signals";
import { useEffect } from "preact/hooks";
import { PocketPaper } from "../../utils/form-state.tsx";

export function Importer() {

    function handleImport() {
        document.getElementById("fileElem")!.click()
    }

    useEffect(() => {
        document.getElementById("fileElem")!.addEventListener("change", (ev) => {
            const file = ev.currentTarget! as HTMLInputElement
            const fs = new FileReader()
            fs.onloadend = (ctx) => {
                if(ctx.target!.readyState === ctx.target!.DONE) {
                    const urlQuery = new URLSearchParams(JSON.parse(ctx.target!.result!.toString()))
                    globalThis.location.replace("/import-existing?" + urlQuery.toString())
                }
            }
            fs.readAsText(file.files![0])
        })
    })

    return(
        <>
        <a type="button" class="index-link" onClick={handleImport}>Importieren</a>
        <input
        type="file"
        id="fileElem"
        multiple={false}
        accept="application/json,.pocketpaper"
        style="display:none" />
        </>
    )

}

export function FormPropagator(props: { 
    page: PageProps,  
    requestRecalc: Signal<boolean | undefined>,
    tableValues: Signal<PocketPaper | undefined>
}) {

    useEffect(() => {

        const elements = document.querySelector("form")!.elements
        for(const [key, val] of new URLSearchParams(new URL(props.page.url).search)) {
            const field = elements.namedItem(key)! as Element
            if (field.id.includes("discipline")) {
                const fieldCast = field as HTMLSelectElement
                fieldCast.value = val
            } else if (field.id.includes("agegroup")) {
                const fieldCast = field as HTMLSelectElement
                fieldCast.value = val
            } else if (field.id.includes("_desc")) {
                const fieldCast = field as HTMLTextAreaElement
                fieldCast.value = val
            } else {
                const fieldCast = field as HTMLInputElement
                fieldCast.value = val
            }
        }

        const values = props.tableValues.value!
        for(let i=0; i < 14; i++) {
            if(i === 13) {
                values.moves.set("Abg", {
                    isDismount: false,
                    abbr: "",
                    desc: "",
                    structureGroups: "",
                    difficultyValue: ""
                  })
            } else {
                values.moves.set(i.toString(), {
                    isDismount: false,
                    abbr: "",
                    desc: "",
                    structureGroups: "",
                    difficultyValue: ""
                  })
            }
        }
        for(const [key, val] of new URLSearchParams(new URL(props.page.url).search)) {
            if(key === "discipline") {
                //@ts-ignore: Is extracted
                values.discipline = val
            } else {
                const no = key.split("_")
                if(no[0] === "Abg") {
                    values.moves.get(no[0])!.isDismount = true
                }
                switch (no[1]) {
                    case "abbr":
                        values.moves.get(no[0])!.abbr = val
                        break
                    case "desc":
                        values.moves.get(no[0])!.desc = val
                        break
                    case "sgs":
                        values.moves.get(no[0])!.structureGroups = val
                        break
                    case "diff":
                        values.moves.get(no[0])!.difficultyValue = val
                        break
                }
            }
        }
        props.tableValues.value = Object.assign({}, values)
        props.requestRecalc.value = true

    })

    return(
        <>
        </>
    )
}