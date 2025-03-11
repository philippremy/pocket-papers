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
      "Hosentaschenkarte\nSprung",
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
  columns: (7.5%, 20%, 53.5%, 20%),
  rows: (35pt, 30%, 30%),
  table.header(
    [*Nr.*],
    [*Teil*],
    [*Beschreibung*],
    [*Wertigkeit*]
    ),
    [*#align(center + horizon, "1.")*], align(center + horizon, abbr_1), align(horizon, fill-height-with-text(desc_1)), [#align(center + horizon, text(diff_1, weight: "bold"))],
    
    [*#align(center + horizon, "2.")*], align(center + horizon, abbr_2), align(horizon, fill-height-with-text(desc_2)), [#align(center + horizon, text(diff_2, weight: "bold"))],
  )
)
)