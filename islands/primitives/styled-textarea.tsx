import { ComponentChildren } from "preact/src/index.d.ts";

export function StyledTextArea(props: {
    type?: string,
    children?: ComponentChildren,
    name: string,
    required?: boolean,
    form?: string,
    value?: string,
    onInput?: (newValue: string) => void,
    onChange?: (target: HTMLDivElement) => void,
}) {

    return(
    <div class="styled-input-container" onClick={(ctx) => {
        (ctx.currentTarget.children[0] as HTMLDivElement).focus()
    }}>
        <div 
            contenteditable 
            class="styled-text-area" 
            onInput={(ctx) => {
                const text = ctx.currentTarget.innerText
                
                const hiddenTextarea = ctx.currentTarget.querySelector("textarea"); // Find the textarea
                if (hiddenTextarea) {
                    hiddenTextarea.value = text; // Update textarea value
                }

                props.onInput?.(text); // Pass the value to the parent component
            }}
        >
            <textarea 
                hidden
                id={props.name} 
                name={props.name} 
                form={props.form} 
                required={props.required} 
                type={props.type} 
                value={props.value} 
                onSubmit={(ctx) => {
                    ctx.currentTarget.value = "";
                    ctx.currentTarget.parentElement!.dataset.value = "";
                }} 
            />
        </div>
        <label for={props.name} class="styled-textarea-label noedit">{props.children}</label>
    </div>
    )

}