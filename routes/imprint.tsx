import BasicHead from "../components/meta/basic-head.tsx";
import NavBar, { NavBarTextLink } from "../components/navbar/navbar.tsx";

export default function Imprint() {
    return(
        <>
        <BasicHead />
        <link rel="stylesheet" href="/imprint.css" />
        <NavBar>
            <NavBarTextLink text="Hosentaschenkarten" href="/" />
            <NavBarTextLink text="DTB Rhönradturnen" href="https://www.dtb.de/rhoenradturnen/" />
            <NavBarTextLink text="Rhönrad Events" href="https://rhoenrad.events" />
            <NavBarTextLink text="Wertungsbestimmungen" href="https://www.dtb.de/rhoenradturnen/rhoenrad/wertungsbestimmungen" />
            <NavBarTextLink text="Datenschutz" href="/privacy" />
            <NavBarTextLink text="Impressum" href="/imprint" />
        </NavBar>
        <main class="imprint-main-container">
            <h1>IMPRESSUM</h1>
            <h2>Angaben nach § 5 Digitale-Dienste-Gesetz (DDG)</h2>
            <div class="imprint-inner-container">
                <p>Philipp Remy</p>
                <p>Philipsstraße 10</p>
                <p>20099 Hamburg</p>
            </div>
            <h4>Kontakt:</h4>
            <div class="imprint-inner-container">
                <p>E-Mail: <a href="mailto:philipp.remy@dtb.de">philipp.remy@dtb.de</a></p>
            </div>
            <h4>Verantwortlich für den Inhalt der Website (§ 18 Abs. 2 Medienstaatsvertrag [MStV]):</h4>
            <div class="imprint-inner-container">
                <p>Philipp Remy</p>
                <p>Philipsstraße 10</p>
                <p>20099 Hamburg</p>
            </div>
        </main>
        </>
    )
}