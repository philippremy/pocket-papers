
export default function Index() {
    return(
        <>
        <Head>
          <BasicHead />
          <link rel="stylesheet" href="/index.css" />
          <link rel="stylesheet" href="/gradient.css" />
        </Head>
        <div>
          <NavBar>
            <NavBarTextLink text="DTB Rhönradturnen" href="https://www.dtb.de/rhoenradturnen/" />
            <NavBarTextLink text="Rhönrad Events" href="https://rhoenrad.events" />
            <NavBarTextLink text="Wertungsbestimmungen" href="https://www.dtb.de/rhoenradturnen/rhoenrad/wertungsbestimmungen" />
            <NavBarTextLink text="Datenschutz" href="/privacy" />
            <NavBarTextLink text="Impressum" href="/imprint" />
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
    )
}