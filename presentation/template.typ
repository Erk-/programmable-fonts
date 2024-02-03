#import "metadata.typ": *

#let slides(doc) = {
	// Variables for configuration.
	let scale = 2cm
	let width = beamer_format.at(0) * scale
	let height = beamer_format.at(1) * scale
	
	// Setup.
	set document(
		title: presentation_title,
		author: author,
	)
	set text(
		font: font,
		size: 25pt,
	)
	set page(
		width: width,
		height: height,
		margin: 0pt,
	)
	set align(left + top)
	
	show heading: title => {
		pagebreak(weak: true)
		rect(
			width: 100%, height: 100%,
			fill: gradient.radial(theme_background.lighten(15%), theme_background.darken(15%)),
			pad(x: 10pt, y: 10pt, text(fill: theme_text, title))
		)
		counter(page).update(x => x - 1)
		pagebreak(weak: true)
	}
	
	// Title page.
	rect(
		width: 100%, height: 100%,
		fill: gradient.radial(theme_background.lighten(15%), theme_background.darken(15%)),
                pad(x: 10pt, y: 10pt, 
		{
			set text(fill: theme_text)
			text(size: 50pt, weight: "bold", presentation_title)
			linebreak()
			text(size: 30pt, presentation_subtitle)
			v(2em)
			text(size: 25pt, author)
			linebreak()
			text(size: 15pt, date)
		})
	)
	pagebreak(weak: true)
	counter(page).update(1)
	
	// Actual content.
	doc
}

#let slide(title: "", content) = locate(loc => {
	// Header with slide title.
	let header = {
		let headers = query(selector(heading).before(loc), loc)
		set align(left + top)
		set text(fill: theme_text, weight: "bold")
		rect(width: 100%, fill: theme_background, pad(x: 10pt, y: 10pt)[
			#if headers == () {
				text(size: 30pt, title)
			} else {
				let section = headers.last().body
				if title == "" {
					text(size: 30pt, section)
				} else {
					text(size: 15pt, section)
					linebreak()
					text(size: 25pt, title)
				}
			}
		])
	}
	
	// Footer with left and right section.
	let footer = grid(columns: (1fr, auto), pad(x: 5pt, y: 8pt)[
		// Presentation title and author.
		#set align(left)
		#set text(12pt)
		#presentation_title \
		#set text(10pt)
		#author -- #date
	], [
		// Page counter.
		#rect(
			width: 60pt,
			height: 40pt,
			fill: theme_background,
			align(
				center + horizon,
				text(20pt, fill: theme_text, counter(page).display())
			)
		)
	])
	
	pagebreak(weak: true)
	grid(rows: (auto, 1fr, auto), {
		header
	}, {
		// Inner slide content.
		pad(x: 10pt, y: 10pt, box(align(left, content)))
	}, {
		footer
	})
	pagebreak(weak: true)
})
