#import "conf.typ": *

#set page(
  paper: "a4",
  margin: (bottom: 10pt, top: 25pt, left: 30pt, right: 30pt)
)

#set text(font: "Noto Sans")


#table(
  columns: (35%, 50%, 15%),
  stroke: none,
  image(
    "dtb.svg",
    width: 90%,
    fit: "contain"
  ),
  align(
    center,
    text(
      "Hosentaschenkarte Geradeturnen",
      weight: "black",
      size: 18pt,
    )
  ),
  image(
    "abteilung.svg"
  )
)

#move(
  dy: -25pt,
  table(
    columns: (33%, 66%),
    stroke: none,
    text(
      "Name, Vorname:",
      weight: "semibold",
      size: 12pt,
    ),
    text(
      name,
      size: 12pt,
    ),
      text(
      "Verein:",
      weight: "semibold",
      size: 12pt,
    ),
    text(
      club,
      size: 12pt,
    ),
      text(
      "Altersklasse:",
      weight: "semibold",
      size: 12pt,
    ),
    text(
      age_group,
      size: 12pt,
    )
  )
)

#set table(stroke: 0.75pt)

#box(
  height: 640pt,
  move(
  dy: -15pt,
  table(
  fill: (x, y) =>
    if y == 0 {
      gray.lighten(70%)
    },
  columns: (7.5%, 12.5%, 42.5%, 25%, 12.5%),
  rows: (35pt, 1fr, 1fr, 1fr, 1fr, 1fr, 1fr, 1fr, 1fr, 1fr, 1fr, 1fr, 1fr, 1fr, 35pt, 35pt, 35pt),
  table.header(
    [*Nr.*],
    [*Teil*],
    [*Beschreibung*],
    [*Strukturgruppen/Anforderungen*],
    [*Wertigkeit*]
    ),
    [*#align(center + horizon, "1.")*], align(center + horizon, abbr_1), desc_1, [#align(center + horizon, sgs_1)], [#align(center + horizon, text(diff_1, weight: "bold"))],
    
    [*#align(center + horizon, "2.")*], align(center + horizon, abbr_2), desc_2, [#align(center + horizon, sgs_2)], [#align(center + horizon, text(diff_2, weight: "bold"))],
    
    [*#align(center + horizon, "3.")*], align(center + horizon, abbr_3), desc_3, [#align(center + horizon, sgs_3)], [#align(center + horizon, text(diff_3, weight: "bold"))],
    
    [*#align(center + horizon, "4.")*], align(center + horizon, abbr_4), desc_4, [#align(center + horizon, sgs_4)], [#align(center + horizon, text(diff_4, weight: "bold"))],
    
    [*#align(center + horizon, "5.")*], align(center + horizon, abbr_5), desc_5, [#align(center + horizon, sgs_5)], [#align(center + horizon, text(diff_5, weight: "bold"))],
    
    [*#align(center + horizon, "6.")*], align(center + horizon, abbr_6), desc_6, [#align(center + horizon, sgs_6)], [#align(center + horizon, text(diff_6, weight: "bold"))],
    
    [*#align(center + horizon, "7.")*], align(center + horizon, abbr_7), desc_7, [#align(center + horizon, sgs_7)], [#align(center + horizon, text(diff_7, weight: "bold"))],
    
    [*#align(center + horizon, "8.")*], align(center + horizon, abbr_8), desc_8, [#align(center + horizon, sgs_8)], [#align(center + horizon, text(diff_8, weight: "bold"))],
    
    [*#align(center + horizon, "9.")*], align(center + horizon, abbr_9), desc_9, [#align(center + horizon, sgs_9)], [#align(center + horizon, text(diff_9, weight: "bold"))],
    
    [*#align(center + horizon, "10.")*], align(center + horizon, abbr_10), desc_10, [#align(center + horizon, sgs_10)], [#align(center + horizon, text(diff_10, weight: "bold"))],
    
    [*#align(center + horizon, "11.")*], align(center + horizon, abbr_11), desc_11, [#align(center + horizon, sgs_11)], [#align(center + horizon, text(diff_11, weight: "bold"))],
    
    [*#align(center + horizon, "12.")*], align(center + horizon, abbr_12), desc_12, [#align(center + horizon, sgs_12)], [#align(center + horizon, text(diff_12, weight: "bold"))],

    [*#align(center + horizon, "Abg.")*], align(center + horizon, abbr_abg), desc_abg, [#align(center + horizon, sgs_abg)], [#align(center + horizon, text(diff_abg, weight: "bold"))],
    
    table.cell(colspan: 2, [*#align(horizon, text("Höchste 8 Teile:"))*], stroke: (top: 2pt)), table.cell(colspan: 3, align(horizon, table(
      columns: (10%, 10%, 10%, 10%, 10%, 10%, 10%, 10%, 10%),
      rows: (50%, 50%),
      column-gutter: 1.2%,
      stroke: none,
      sg_1,
      sg_2,
      sg_3,
      sg_4,
      sg_5,
      sg_6,
      sg_7,
      sg_8,
      sg_9,
      sg_10,
      sg_11,
      sg_12,
      sg_13,
      sg_14,
      sg_15,
      sg_16,
      sg_17
    )), stroke: (top: 2pt)),
    table.cell(colspan: 2, [*#align(horizon, text("Strukturgruppen/Anforderungen:"))*]), table.cell(colspan: 3, align(center + horizon, table(
      columns: (15%, 15%, 15%, 15%, 15%, 15%),
      rows: (100%),
      column-gutter: 1.5%,
      stroke: none,
      zero_elem,
      A_elem,
      B_elem,
      C_elem,
      D_elem,
      E_elem
    ))),
    table.cell(colspan: 4, [*#align(horizon, text("Gesamt:"))*], stroke: (top: 2pt)), table.cell(colspan: 1, [*#align(center + horizon, sum_diff)*], stroke: (top: 2pt))
  )
)
)