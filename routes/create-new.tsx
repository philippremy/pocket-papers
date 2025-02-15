import { Head } from "$fresh/runtime.ts";
import { ComponentChildren } from "preact/src/index.d.ts";
import BasicHead from "../components/meta/basic-head.tsx";
import NavBar, { NavBarTextLink } from "../components/navbar/navbar.tsx";
import PocketPaperTable from "../islands/table/table.tsx";
import { iframeStatus, tableAlteredByParent, tableKind, tableValues } from "../utils/form-state.tsx";
import { StyledInput } from "../islands/primitives/styled-input.tsx";
import { StyledSelect } from "../islands/primitives/styled-select.tsx";
import FormComponent, { FormResponse, FormSubmit } from "../islands/data/form.tsx";

export default function CreateNew() {

    return (
        <>
        <Head>
        <BasicHead />
        <link rel="stylesheet" href="/create-new.css" />
        <link rel="stylesheet" href="/gradient.css" />
        </Head>
        <div>
        <NavBar>
            <NavBarTextLink text="DTB Rhönradturnen" href="" />
            <NavBarTextLink text="Rhönrad Events" href="" />
            <NavBarTextLink text="Wertungsbestimmungen" href="" />
            <NavBarTextLink text="Datenschutz" href="" />
            <NavBarTextLink text="Impressum" href="" />
        </NavBar>
        <main class="main-content-container">
            <FormResponse iframeSignal={iframeStatus} name="pocket-paper-response" />
            <div class="gradient" />
            <FormComponent name="pocket-paper" baseRoute="/pocket-paper-api/" method="POST" signal={tableKind} target="pocket-paper-response" />
            <div class="display-divider">
                <div class="mobile-padding" />
                <div class="card-divider">
                    <MainContentCard title="Persönliche Daten">
                        <StyledInput name="name" type="name" required form="pocket-paper">
                            Nachname, Vorname
                        </StyledInput>
                        <StyledInput name="club" type="name" required form="pocket-paper">
                            Verein
                        </StyledInput>
                        <StyledSelect name="agegroup" placeholder="Altersklasse" required form="pocket-paper">
                            <option selected disabled style="display:none"></option>
                            <optgroup label="Nachwuchsklasse">
                                <option>AK N 7/8</option>
                                <option>AK N 9/10</option>
                                <option>AK N 11/12</option>
                            </optgroup>
                            <optgroup label="Landesklasse">
                                <option>AK L 13/14</option>
                                <option>AK L 15/16</option>
                                <option>AK L 17/18</option>
                                <option>AK L 19+</option>
                                <option>AK L 25+</option>
                                <option>AK L 30+</option>
                                <option>AK L 40+</option>
                            </optgroup>
                            <optgroup label="Bundesklasse">
                                <option>AK B 12</option>
                                <option>AK B 13/14</option>
                                <option>AK B 15/16</option>
                                <option>AK B 17/18</option>
                                <option>AK B 19+</option>
                            </optgroup>
                        </StyledSelect>
                    </MainContentCard>
                    <MainContentCard title="Funktionen">
                        <FormSubmit iframeSignal={iframeStatus} form="pocket-paper" />
                    </MainContentCard>
                </div>
                <MainContentCard title="Hosentaschenkarte">
                    <PocketPaperTable tableKind={tableKind} tableValues={tableValues} tableAlteredByParent={tableAlteredByParent} form="pocket-paper" />
                </MainContentCard>
                <div class="bottom-padding">
                </div>
            </div>
        </main>
        </div>
        </>
    );
}

export function MainContentCard(props: {
    title?: string,
    children?: ComponentChildren
}) {

    return(
        <div class="main-content-card-container">
            <h2 class="main-content-card-title">{props.title}</h2>
            {props.children}
        </div>
    )

}