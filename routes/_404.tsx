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
        <NavBarTextLink text="DTB Rhönradturnen" href="" />
        <NavBarTextLink text="Rhönrad Events" href="" />
        <NavBarTextLink text="Wertungsbestimmungen" href="" />
        <NavBarTextLink text="Datenschutz" href="" />
        <NavBarTextLink text="Impressum" href="" />
      </NavBar>
      <div class="fourofour-content">
        <h1 class="fourofour-big-questionmark">?</h1>
        <p class="fourofour-heading">404 - Seite nicht gefunden</p>
        <p class="fourofour-caption">Ich glaube, wir sind hier falsch...</p>
      </div>
    </>
  );
}
