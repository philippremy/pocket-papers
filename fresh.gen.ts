// DO NOT EDIT. This file is generated by Fresh.
// This file SHOULD be checked into source version control.
// This file is automatically updated during development when running `dev.ts`.

import * as $_404 from "./routes/_404.tsx";
import * as $_app from "./routes/_app.tsx";
import * as $create_new from "./routes/create-new.tsx";
import * as $imprint from "./routes/imprint.tsx";
import * as $index from "./routes/index.tsx";
import * as $privacy from "./routes/privacy.tsx";
import * as $data_form from "./islands/data/form.tsx";
import * as $primitives_styled_input from "./islands/primitives/styled-input.tsx";
import * as $primitives_styled_select from "./islands/primitives/styled-select.tsx";
import * as $primitives_styled_textarea from "./islands/primitives/styled-textarea.tsx";
import * as $table_table from "./islands/table/table.tsx";
import type { Manifest } from "$fresh/server.ts";

const manifest = {
  routes: {
    "./routes/_404.tsx": $_404,
    "./routes/_app.tsx": $_app,
    "./routes/create-new.tsx": $create_new,
    "./routes/imprint.tsx": $imprint,
    "./routes/index.tsx": $index,
    "./routes/privacy.tsx": $privacy,
  },
  islands: {
    "./islands/data/form.tsx": $data_form,
    "./islands/primitives/styled-input.tsx": $primitives_styled_input,
    "./islands/primitives/styled-select.tsx": $primitives_styled_select,
    "./islands/primitives/styled-textarea.tsx": $primitives_styled_textarea,
    "./islands/table/table.tsx": $table_table,
  },
  baseUrl: import.meta.url,
} satisfies Manifest;

export default manifest;
