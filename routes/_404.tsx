import { Head } from "$fresh/runtime.ts";
import BasicHead from "../components/meta/basic-head.tsx";
import NavBar, { NavBarTextLink } from "../components/navbar/navbar.tsx";

export default function Error404() {
  return (
    <>
      <Head>
        <title>404 - Seite nicht gefunden</title>
        <BasicHead />
        <link rel="stylesheet" href="/404.css" ></link>
      </Head>
      <NavBar>
        <NavBarTextLink text="Hosentaschenkarten" href="/" />
        <NavBarTextLink text="DTB Rhönradturnen" href="https://www.dtb.de/rhoenradturnen/" />
        <NavBarTextLink text="Rhönrad Events" href="https://rhoenrad.events" />
        <NavBarTextLink text="Wertungsbestimmungen" href="https://www.dtb.de/rhoenradturnen/rhoenrad/wertungsbestimmungen" />
        <NavBarTextLink text="Datenschutz" href="/privacy" />
        <NavBarTextLink text="Impressum" href="/imprint" />
      </NavBar>
      <div class="fourofour-content">
        <h1 class="fourofour-big-questionmark">?</h1>
        <p class="fourofour-heading">404 - Seite nicht gefunden</p>
        <p class="fourofour-caption">Ich glaube, wir sind hier falsch...</p>
      </div>
    </>
  );
}
