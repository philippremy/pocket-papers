import { type PageProps } from "$fresh/server.ts";
export default function App({ Component }: PageProps) {
  return (
    <html lang="de">
      <head>
        <meta charset="utf-8" />
        <title>Pocket Papers</title>
      </head>
      <body>
        <Component />
      </body>
    </html>
  );
}
