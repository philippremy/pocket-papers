import { PageProps } from "$fresh/server.ts";
import { Head } from "$fresh/runtime.ts";
import BasicHead from "../components/meta/basic-head.tsx";
import IFRAMEResponse from "../islands/data/iframe-response-island.tsx";
import { iframeStatus } from "../utils/form-state.tsx";

export default function IFRAMEResponseRoute(
    page: PageProps
) {
    return (
      <>
        <Head>
            <BasicHead />
            <meta name="color-scheme" content="light dark" />
            <link rel="stylesheet" href="/index.css" />
            <link rel="stylesheet" href="/iframe-response.css" />
        </Head>
        <div>
            <main class="main-iframe-container">
                <IFRAMEResponse page={page} signal={iframeStatus} />
            </main>
        </div>
      </>
    );
}