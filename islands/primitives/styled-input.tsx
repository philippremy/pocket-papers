import { ComponentChildren } from "preact/src/index.d.ts";

export function StyledInput(props: {
    type?: string,
    children?: ComponentChildren,
    name: string,
    required?: boolean,
    form?: string,
    value?: string,
    disabled?: boolean     
    onInput?: (newValue: string) => void,
    onChange?: (target: HTMLInputElement) => void,
}) {

    return(
        <div class="styled-input-container" onClick={(ctx) => {
            (ctx.currentTarget.children[0] as HTMLInputElement).focus()
        }}>
            <input disabled={props.disabled} id={props.name} name={props.name} type={props.type ? props.type : "text"} class="styled-input" required={props.required} form={props.form} value={props.value} onInput={(ctx) => {
                props.onInput ? props.onInput(ctx.currentTarget.value) : {}
            }} onChange={(ctx) => {
                props.onChange ? props.onChange(ctx.currentTarget) : {}
            }} autoComplete="off" />
            <label for={props.name} class="styled-input-label noedit">{props.children}</label>
        </div>
    )

}