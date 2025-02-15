import { Head } from "$fresh/runtime.ts";
import BasicHead from "../components/meta/basic-head.tsx";
import NavBar, { NavBarTextLink } from "../components/navbar/navbar.tsx";

export default function Index() {
  return (
    <>
    <Head>
      <BasicHead />
      <link rel="stylesheet" href="/index.css" />
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
        <div class="gradient" />
        <h1 class="index-heading">Hosentaschenkarten<br />Rhönradturnen</h1>
        <div class="index-link-container">
          <a class="index-link" href="/create-new">Neu erstellen</a>
          <a class="index-link" href="/import-existing">Importieren</a>
        </div>
      </main>
    </div>
    </>
  );
}
