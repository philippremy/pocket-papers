#import "conf.typ": *

#let fill-height-with-text(min: 0.3em, max: 1em, eps: 0.1em, it) = layout(size => {
  let fits(text-size, it) = {
    measure(width: size.width, { set text(text-size); it }).height <= size.height
  }

  if not fits(min, it) { panic("Content doesn't fit even at minimum text size") }
  if fits(max, it) { set text(max); }

  let (a, b) = (min, max)
  while b - a > eps {
    let new = 0.5 * (a + b)
    if fits(new, it) {
      a = new
    } else {
      b = new
    }
  }

  set text(a)
  it
})

#set page(
  paper: "a4",
  margin: (bottom: 10pt, top: 25pt, left: 30pt, right: 30pt)
)

#set text(font: "Arial")


#table(
  columns: (35%, 50%, 15%),
  stroke: none,
  image(
    "../Shared/dtb.svg",
    width: 90%,
    fit: "contain"
  ),
  align(
    center,
    text(
      "Hosentaschenkarte Spiraleturnen",
      weight: "black",
      size: 18pt,
    )
  ),
  image(
    "../Shared/abteilung.svg"
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
    [*#align(center + horizon, "1.")*], align(center + horizon, abbr_1), align(horizon, fill-height-with-text(desc_1)), [#align(center + horizon, sgs_1)], [#align(center + horizon, text(diff_1, weight: "bold"))],
    
    [*#align(center + horizon, "2.")*], align(center + horizon, abbr_2), align(horizon, fill-height-with-text(desc_2)), [#align(center + horizon, sgs_2)], [#align(center + horizon, text(diff_2, weight: "bold"))],
    
    [*#align(center + horizon, "3.")*], align(center + horizon, abbr_3), align(horizon, fill-height-with-text(desc_3)), [#align(center + horizon, sgs_3)], [#align(center + horizon, text(diff_3, weight: "bold"))],
    
    [*#align(center + horizon, "4.")*], align(center + horizon, abbr_4), align(horizon, fill-height-with-text(desc_4)), [#align(center + horizon, sgs_4)], [#align(center + horizon, text(diff_4, weight: "bold"))],
    
    [*#align(center + horizon, "5.")*], align(center + horizon, abbr_5), align(horizon, fill-height-with-text(desc_5)), [#align(center + horizon, sgs_5)], [#align(center + horizon, text(diff_5, weight: "bold"))],
    
    [*#align(center + horizon, "6.")*], align(center + horizon, abbr_6), align(horizon, fill-height-with-text(desc_6)), [#align(center + horizon, sgs_6)], [#align(center + horizon, text(diff_6, weight: "bold"))],
    
    [*#align(center + horizon, "7.")*], align(center + horizon, abbr_7), align(horizon, fill-height-with-text(desc_7)), [#align(center + horizon, sgs_7)], [#align(center + horizon, text(diff_7, weight: "bold"))],
    
    [*#align(center + horizon, "8.")*], align(center + horizon, abbr_8), align(horizon, fill-height-with-text(desc_8)), [#align(center + horizon, sgs_8)], [#align(center + horizon, text(diff_8, weight: "bold"))],
    
    [*#align(center + horizon, "9.")*], align(center + horizon, abbr_9), align(horizon, fill-height-with-text(desc_9)), [#align(center + horizon, sgs_9)], [#align(center + horizon, text(diff_9, weight: "bold"))],
    
    [*#align(center + horizon, "10.")*], align(center + horizon, abbr_10),align(horizon, fill-height-with-text(desc_10)), [#align(center + horizon, sgs_10)], [#align(center + horizon, text(diff_10, weight: "bold"))],
    
    [*#align(center + horizon, "11.")*], align(center + horizon, abbr_11), align(horizon, fill-height-with-text(desc_11)), [#align(center + horizon, sgs_11)], [#align(center + horizon, text(diff_11, weight: "bold"))],
    
    [*#align(center + horizon, "12.")*], align(center + horizon, abbr_12), align(horizon, fill-height-with-text(desc_12)), [#align(center + horizon, sgs_12)], [#align(center + horizon, text(diff_12, weight: "bold"))],

    [*#align(center + horizon, "Abg.")*], align(center + horizon, abbr_abg), align(horizon, fill-height-with-text(desc_abg)), [#align(center + horizon, sgs_abg)], [#align(center + horizon, text(diff_abg, weight: "bold"))],
    
    table.cell(colspan: 2, [*#align(horizon, text("Höchste 8 Teile:"))*], stroke: (top: 2pt)), table.cell(colspan: 3, align(horizon, table(
      columns: (10%, 10%, 10%, 10%, 10%, 10%, 10%, 10%, 10%),
      rows: (50%, 50%),
      column-gutter: 1.2%,
      stroke: none,
      text(font: ("Twitter Color Emoji", "Arial"), size: 0.9em, sg_1),
      text(font: ("Twitter Color Emoji", "Arial"), size: 0.9em, sg_2),
      text(font: ("Twitter Color Emoji", "Arial"), size: 0.9em, sg_3),
      text(font: ("Twitter Color Emoji", "Arial"), size: 0.9em, sg_4),
      text(font: ("Twitter Color Emoji", "Arial"), size: 0.9em, sg_5),
      text(font: ("Twitter Color Emoji", "Arial"), size: 0.9em, sg_6),
      text(font: ("Twitter Color Emoji", "Arial"), size: 0.9em, sg_7),
      text(font: ("Twitter Color Emoji", "Arial"), size: 0.9em, sg_8),
      text(font: ("Twitter Color Emoji", "Arial"), size: 0.9em, sg_9),
      text(font: ("Twitter Color Emoji", "Arial"), size: 0.9em, sg_10),
      text(font: ("Twitter Color Emoji", "Arial"), size: 0.9em, sg_11),
      text(font: ("Twitter Color Emoji", "Arial"), size: 0.9em, sg_12),
      text(font: ("Twitter Color Emoji", "Arial"), size: 0.9em, sg_13),
      text(font: ("Twitter Color Emoji", "Arial"), size: 0.9em, sg_14),
      text(font: ("Twitter Color Emoji", "Arial"), size: 0.9em, sg_15),
      text(font: ("Twitter Color Emoji", "Arial"), size: 0.9em, sg_16)
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