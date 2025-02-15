import { Handlers, PageProps } from "$fresh/server.ts";
import { Head } from "$fresh/runtime.ts";
import BasicHead from "../../components/meta/basic-head.tsx";
import IFRAMEResponse from "../../islands/data/iframe-response.tsx";
import { iframeStatus } from "../../utils/form-state.tsx";

export const handler: Handlers = {
    async POST(req, ctx) {
        const resp = await ctx.render([...(await req.formData())])
        return resp;
    },
};

export default function IFRAMERequest(props: PageProps) {
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
                <IFRAMEResponse page={props} signal={iframeStatus} />
            </main>
        </div>
      </>
    );
}