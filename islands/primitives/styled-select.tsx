import { ComponentChildren } from "preact/src/index.d.ts";

export function StyledSelect(props: {
    children?: ComponentChildren,
    name: string,
    placeholder?: string,
    required?: boolean,
    form?: string,
    value?: string,
    onSelect?: (newSelection: string) => void,
}) {
    return(
        <div class="styled-select-container" onClick={(ctx) => {
            (ctx.currentTarget.children[0] as HTMLSelectElement).focus()
        }}>
            <select id={props.name} name={props.name} class="styled-select" required={props.required} form={props.form} value={props.value} onChange={(ctx) => {
                props.onSelect ? props.onSelect(ctx.currentTarget.value) : {}
            }}>
                {props.children}
            </select>
            <label for={props.name} class="styled-select-label noedit">{props.placeholder}</label>
        </div>
    )
}