import { Signal } from "@preact/signals";
import { PocketPaper } from "../../utils/form-state.tsx";
import { Head } from "$fresh/runtime.ts";
import { useEffect } from "preact/hooks";
import { StyledInput } from "../primitives/styled-input.tsx";
import { StyledSelect } from "../primitives/styled-select.tsx";
import { StyledTextArea } from "../primitives/styled-textarea.tsx";
import { ComponentChildren } from "preact/src/index.d.ts";

export default function PocketPaperTable(props: {
    form?: string,
    tableKind: Signal<"Geradeturnen" | "Spiraleturnen" | "Sprung">,
    tableValues: Signal<PocketPaper | undefined>,
    tableAlteredByParent: Signal<boolean>
}) {

    function setFinalDifficulty(_ctx: HTMLInputElement) {
        recalculateDifficulty()
    }

    function recalculateDifficulty() {

        if(props.tableAlteredByParent.value) {
            return
        }

        const difficultyValues = [];

        for(const diffString of [...props.tableValues.value!.moves].map((obj) => { return obj[1].difficultyValue })) {
            // Remove Pkt. or stuff
            const numbersPktRemoved = diffString.replace(/\b(Pkt\.?|P\.?|P|pkt\.?|p\.?|pkt|p)\b/g, '').trim();

            // We did not succeed? This might be a double value!
            const matchedNumbers = numbersPktRemoved.match(/-?\d+(?:[.,]\d)?/g);

            if(matchedNumbers === null) {
                continue
            }

            for(let matchedNumber of matchedNumbers) {
                // Try to parse
                if (matchedNumber.includes(",")) {
                    matchedNumber = matchedNumber.replace(",", ".")
                }
                if(!Number.isNaN(parseFloat(matchedNumber))) {
                    difficultyValues.push(parseFloat(matchedNumber))
                }
            }
        }

        // Summiere die 8 höchsten Werte
        const eightHighest = difficultyValues.sort((a, b) => b - a).slice(0, Math.min(8, difficultyValues.length));

        const diffMap = new Map<string, number>();
        for(const elemVal of eightHighest) {
            switch (elemVal) {
                case 0.0:
                    if(diffMap.get("0") === undefined) {
                        if(diffMap.get("0") === null) {
                            diffMap.set("0", 1)
                            break
                        }
                        diffMap.set("0", 1)
                        break
                    }
                    diffMap.set("0", diffMap.get("0")! + 1)
                    break
                case 0.2:
                    if(diffMap.get("A") === undefined) {
                        if(diffMap.get("A") === null) {
                            diffMap.set("A", 1)
                            break
                        }
                        diffMap.set("A", 1)
                        break
                    }
                    diffMap.set("A", diffMap.get("A")! + 1)
                    break
                case 0.4:
                    if(diffMap.get("B") === undefined) {
                        if(diffMap.get("B") === null) {
                            diffMap.set("B", 1)
                            break
                        }
                        diffMap.set("B", 1)
                        break
                    }
                    diffMap.set("B", diffMap.get("B")! + 1)
                    break
                case 0.6:
                    if(diffMap.get("C") === undefined) {
                        if(diffMap.get("C") === null) {
                            diffMap.set("C", 1)
                            break
                        }
                        diffMap.set("C", 1)
                        break
                    }
                    diffMap.set("C", diffMap.get("C")! + 1)
                    break
                case 0.8:
                    if(diffMap.get("D") === undefined) {
                        if(diffMap.get("D") === null) {
                            diffMap.set("D", 1)
                            break
                        }
                        diffMap.set("D", 1)
                        break
                    }
                    diffMap.set("D", diffMap.get("D")! + 1)
                    break
                case 1.0:
                    if(diffMap.get("E") === undefined) {
                        if(diffMap.get("E") === null) {
                            diffMap.set("E", 1)
                            break
                        }
                        diffMap.set("E", 1)
                        break
                    }
                    diffMap.set("E", diffMap.get("E")! + 1)
                    break
                default:
                    break
            }
        }
        props.tableValues.value!.highestElements = diffMap

        const sumOf8Highest = eightHighest.reduce((sum, num) => sum + num, 0);

        props.tableValues.value!.finalDifficulty = sumOf8Highest
        props.tableValues.value! = Object.assign({}, props.tableValues.value!)

    }

    function romanToArabic(roman: string): number | null {
        const romanMap = {
            "I": 1, "II": 2, "III": 3, "IV": 4, "V": 5,
            "VI": 6, "VII": 7, "VIII": 8, "IX": 9, "X": 10,
            "XI": 11, "XII": 12, "XIII": 13, "XIV": 14, "XV": 15,
            "XVI": 16, "XVII": 17,
    
            // Lowercase versions
            "i": 1, "ii": 2, "iii": 3, "iv": 4, "v": 5,
            "vi": 6, "vii": 7, "viii": 8, "ix": 9, "x": 10,
            "xi": 11, "xii": 12, "xiii": 13, "xiv": 14, "xv": 15,
            "xvi": 16, "xvii": 17
        }
    
        // @ts-ignore This is fine i guess
        return romanMap[roman] || null;
    }

    function recalculateStructureGroups() {

        if(props.tableAlteredByParent.value) {
            return
        }

        const diffMap = new Map([
            [1, false],
            [2, false],
            [3, false],
            [4, false],
            [5, false],
            [6, false],
            [7, false],
            [8, false],
            [9, false],
            [10, false],
            [11, false],
            [12, false],
            [13, false],
            [14, false],
            [15, false],
            [16, false],
            (props.tableKind.value === "Geradeturnen" ? [17, false] : [1, false])
        ]);
        for(const structureGroupString of [...props.tableValues.value!.moves].map((obj) => { return obj[1].structureGroups })) {
                
            const matches = structureGroupString.match(/(XVII|XVI|XV|XIV|XIII|XII|XI|X|IX|VIII|VII|VI|V|IV|III|II|I|xvii|xvi|xv|xiv|xiii|xii|xi|x|ix|viii|vii|vi|v|iv|iii|ii|i|1[0-7]|\d+)/g);
        
            const foundNumbers = matches ? matches.map(num => (Number.isNaN(parseInt(num)) ? romanToArabic(num) : parseInt(num))) : [];

            for(const number of foundNumbers) {
                if(number === null) {
                    continue
                }
                if(number === 17 && props.tableKind.value !== "Geradeturnen") {
                    continue
                }
                if(number > 17) {
                    continue
                }
                diffMap.set(number, true)
            }

        }

        props.tableValues.value!.fulfilledStructureGroups = diffMap

    }

    useEffect(() => {
        if(props.tableValues.value === undefined) {
            props.tableValues.value = {
                discipline: props.tableKind.value,
                moves: new Map(),
                fulfilledStructureGroups: new Map([
                    [1, false],
                    [2, false],
                    [3, false],
                    [4, false],
                    [5, false],
                    [6, false],
                    [7, false],
                    [8, false],
                    [9, false],
                    [10, false],
                    [11, false],
                    [12, false],
                    [13, false],
                    [14, false],
                    [15, false],
                    [16, false],
                    (props.tableKind.value === "Geradeturnen" ? [17, false] : [1, false])
                ]),
                finalDifficulty: 0.0,
                highestElements: new Map([
                    ["0", null],
                    ["A", null],
                    ["B", null],
                    ["C", null],
                    ["D", null],
                    ["E", null]
                ])
            }
        }
    })

    useEffect(() => {
        props.tableValues.value = {
            discipline: props.tableKind.value,
            moves: new Map(),
            fulfilledStructureGroups: new Map([
                [1, false],
                [2, false],
                [3, false],
                [4, false],
                [5, false],
                [6, false],
                [7, false],
                [8, false],
                [9, false],
                [10, false],
                [11, false],
                [12, false],
                [13, false],
                [14, false],
                [15, false],
                [16, false],
                (props.tableKind.value === "Geradeturnen" ? [17, false] : [1, false])
            ]),
            finalDifficulty: 0.0,
            highestElements: new Map([
                ["0", null],
                ["A", null],
                ["B", null],
                ["C", null],
                ["D", null],
                ["E", null]
            ])
        }
        props.tableValues.value = Object.assign({}, props.tableValues.value!)
    }, [props.tableKind.value])

    useEffect(() => {
        recalculateDifficulty()
        recalculateStructureGroups()
        props.tableAlteredByParent.value = true
    }, [props.tableValues.value])

    return(
        <div class="table-top-container">
            <Head>
                <link rel="stylesheet" href="/table.css" />
            </Head>
            <StyledSelect name="discipline" placeholder="Disziplin" required form="pocket-paper" value={props.tableKind.value} onSelect={(newSelection) => {
                // FIXME!
                // @ts-ignore Options are predefined, so this is fine
                props.tableKind.value = newSelection
            }}>
                <option>Geradeturnen</option>                
                <option>Spiraleturnen</option>
                <option>Sprung</option>
            </StyledSelect>
            <div class="table-container">
                { props.tableKind.value === "Geradeturnen" || props.tableKind.value === "Spiraleturnen" ?
                <> 
                <TableEntry no="1" values={props.tableValues} form={props.form} discipline={props.tableKind} alteredByParent={props.tableAlteredByParent} />
                <TableEntry no="2" values={props.tableValues} form={props.form} discipline={props.tableKind} alteredByParent={props.tableAlteredByParent} />
                <TableEntry no="3" values={props.tableValues} form={props.form} discipline={props.tableKind} alteredByParent={props.tableAlteredByParent} />
                <TableEntry no="4" values={props.tableValues} form={props.form} discipline={props.tableKind} alteredByParent={props.tableAlteredByParent} />
                <TableEntry no="5" values={props.tableValues} form={props.form} discipline={props.tableKind} alteredByParent={props.tableAlteredByParent} />
                <TableEntry no="6" values={props.tableValues} form={props.form} discipline={props.tableKind} alteredByParent={props.tableAlteredByParent} />
                <TableEntry no="7" values={props.tableValues} form={props.form} discipline={props.tableKind} alteredByParent={props.tableAlteredByParent} />
                <TableEntry no="8" values={props.tableValues} form={props.form} discipline={props.tableKind} alteredByParent={props.tableAlteredByParent} />
                <TableEntry no="9" values={props.tableValues} form={props.form} discipline={props.tableKind} alteredByParent={props.tableAlteredByParent} />
                <TableEntry no="10" values={props.tableValues} form={props.form} discipline={props.tableKind} alteredByParent={props.tableAlteredByParent} />
                <TableEntry no="11" values={props.tableValues} form={props.form} discipline={props.tableKind} alteredByParent={props.tableAlteredByParent} />
                <TableEntry no="12" values={props.tableValues} form={props.form} discipline={props.tableKind} alteredByParent={props.tableAlteredByParent} />
                <TableEntry no="Abg" values={props.tableValues} form={props.form} discipline={props.tableKind} alteredByParent={props.tableAlteredByParent} />
                </>
                    : // Wenn Sprung
                <>
                <TableEntry no="1" values={props.tableValues} form={props.form} discipline={props.tableKind} alteredByParent={props.tableAlteredByParent} />
                <TableEntry no="2" values={props.tableValues} form={props.form} discipline={props.tableKind} alteredByParent={props.tableAlteredByParent} />
                </>
                }
            </div>
            { props.tableKind.value === "Geradeturnen" || props.tableKind.value === "Spiraleturnen" ?
            <>
            <div class="table-summary-container">
                <h3>Gesamt</h3>
                <TableSummaryStructureGroups values={props.tableValues}>
                    Strukturgruppen
                </TableSummaryStructureGroups>
                <TableSummaryHighestElements values={props.tableValues}>
                    Höchste 8 Teile
                </TableSummaryHighestElements>
                <StyledInput name="difficultySum" value={props.tableValues.value?.finalDifficulty.toFixed(1)} onChange={setFinalDifficulty}>
                    Gesamtschwierigkeit
                </StyledInput>
            </div>
            </>
            : // Wenn Sprung
            <>
            </>
            }
        </div>
    )

}

export function TableSummaryHighestElements(props: {
    children: ComponentChildren
    values: Signal<PocketPaper | undefined>,
}) {

    return(
        <div class="styled-summary-container-elements">
            <span class="styled-textarea-label">{props.children}</span>
            { props.values.value === undefined ?
            <>
            <p>? x <b>0</b></p>
            <p>? x <b>A</b></p>
            <p>? x <b>B</b></p>
            <p>? x <b>C</b></p>
            <p>? x <b>D</b></p>
            <p>? x <b>E</b></p>
            </>
            : // Actual values
            <>
            <p>{props.values.value.highestElements.get("0") ?? "?"} x <b>0</b></p>
            <p>{props.values.value.highestElements.get("A") ?? "?"} x <b>A</b></p>
            <p>{props.values.value.highestElements.get("B") ?? "?"} x <b>B</b></p>
            <p>{props.values.value.highestElements.get("C") ?? "?"} x <b>C</b></p>
            <p>{props.values.value.highestElements.get("D") ?? "?"} x <b>D</b></p>
            <p>{props.values.value.highestElements.get("E") ?? "?"} x <b>E</b></p>
            </>
            }
        </div>
    )

}

export function TableSummaryStructureGroups(props: {
    children: ComponentChildren
    values: Signal<PocketPaper | undefined>,
}) {

    function generateString(ctx: [number, boolean]) {
        switch (ctx[0]) {
            case 1:
                return "I"
            case 2:
                return "II"
            case 3:
                return "III"
            case 4:
                return "IV"
            case 5:
                return "V"
            case 6:
                return "VI"
            case 7:
                return "VII"
            case 8:
                return "VIII"
            case 9:
                return "IX"
            case 10:
                return "X"
            case 11:
                return "XI"
            case 12:
                return "XII"
            case 13:
                return "XIII"
            case 14:
                return "XIV"
            case 15:
                return "XV"
            case 16:
                return "XVI"
            case 17:
                return "XVII"
            default:
                return "N/A"
        }
    }

    return(
        <div class="styled-summary-container">
            <span class="styled-textarea-label">{props.children}</span>
            { props.values.value === undefined ?
            <>
            </>
            : // Actual values
            <>
            {[...props.values.value.fulfilledStructureGroups].map((ctx) => {
                return(
                <div class="summary-checkbox-container">
                <input type="checkbox" checked={ctx[1]} id={ctx[0] + "-sgf"} style={{ pointerEvents: "none" }} />
                <span style={{ pointerEvents: "none" }} for={ctx[0] + "-sgf"}>{generateString(ctx)}</span>
                </div>
                )
            })}
            </>
            }
        </div>
    )

}

export function TableEntry(props: {
    no: string,
    values: Signal<PocketPaper | undefined>,
    form?: string
    discipline: Signal<"Geradeturnen" | "Spiraleturnen" | "Sprung">
    alteredByParent: Signal<boolean>
}) {

    function setAbbr(abbr: string) {
        props.values.value!.moves.set(props.no, props.values.value!.moves.get(props.no) ? props.values.value!.moves.get(props.no)! : {
            isDismount: props.no === "Abg",
            abbr: "",
            desc: "",
            structureGroups: "",
            difficultyValue: ""
        })
        props.values.value!.moves.get(props.no)!.abbr = abbr;
        props.values.value = Object.assign({}, props.values.value!)
        props.alteredByParent.value = false
    }

    function setDesc(desc: string) {
        props.values.value!.moves.set(props.no, props.values.value!.moves.get(props.no) ? props.values.value!.moves.get(props.no)! : {
            isDismount: props.no === "Abg",
            abbr: "",
            desc: "",
            structureGroups: "",
            difficultyValue: ""
        })
        props.values.value!.moves.get(props.no)!.desc = desc;
        props.values.value = Object.assign({}, props.values.value!)
        props.alteredByParent.value = false
    }

    function setStructureGroups(structureGroups: string) {
        props.values.value!.moves.set(props.no, props.values.value!.moves.get(props.no) ? props.values.value!.moves.get(props.no)! : {
            isDismount: props.no === "Abg",
            abbr: "",
            desc: "",
            structureGroups: "",
            difficultyValue: ""
        })
        props.values.value!.moves.get(props.no)!.structureGroups = structureGroups;
        props.values.value = Object.assign({}, props.values.value!)
        props.alteredByParent.value = false
    }

    function setDifficulty(difficulty: string) {
        props.values.value!.moves.set(props.no, props.values.value!.moves.get(props.no) ? props.values.value!.moves.get(props.no)! : {
            isDismount: props.no === "Abg",
            abbr: "",
            desc: "",
            structureGroups: "",
            difficultyValue: ""
        })
        props.values.value!.moves.get(props.no)!.difficultyValue = difficulty;
        props.values.value = Object.assign({}, props.values.value!)
        props.alteredByParent.value = false
    }

    return(
        <div class="table-entry-container">
            <h3>{props.no}.</h3>
            <StyledInput name={props.no + "_abbr"} form={props.form} value={props.values?.value?.moves.get(props.no)?.abbr} onInput={setAbbr}>T-Bz.</StyledInput>
            <StyledTextArea name={props.no + "_desc"} form={props.form} value={props.values?.value?.moves.get(props.no)?.desc} onInput={setDesc}>Beschreibung</StyledTextArea>
            { props.discipline.value !== "Sprung" ?
                <StyledInput name={props.no + "_sgs"} form={props.form} value={props.values?.value?.moves.get(props.no)?.structureGroups} onInput={setStructureGroups}>Strukturgruppen</StyledInput>
                : // Wenn Sprung
                <></>
            }
            <StyledInput name={props.no + "_diff"} form={props.form} value={props.values?.value?.moves.get(props.no)?.difficultyValue} onInput={setDifficulty}>Schwierigkeit</StyledInput>
        </div>
    )

}