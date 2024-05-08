# Notes

-   **info(sky)**: les commentaires `// dfplz`, `// don't format please` et
    autre variantes sont uniquement présents pour que le formater de rust
    n'effectuent pas de transformation sur les parties du code en question.
    N'est pas une règle officielle du formater.

-   **info(sky)**: notre parser HTML n'est pas encore utilisable, on utilise
    pour l'instant `html5ever`.

-   **praise(floem)**: nous pouvons étendre les classes déjà existantes, bon
    point.

-   **issue(floem)**: n'est pour l'instant pas possible d'utiliser de
    l'asynchrone.

-   **issue(floem)**: La fenêtre prend en considération la propriété de style
    `Style.border_radius(N)`, mais l'arrière-plan n'est pas transparent. Ce qui
    pose un souci, `WindowConfig.with_transparent(true)` ne fait qu'empirer le
    souci.

    | WindowConfig.with_transparent(false)       | WindowConfig.with_transparent(true)        |
    | ------------------------------------------ | ------------------------------------------ |
    | ![Fenêtre](docs/issues/win-corner-br1.png) | ![Fenêtre](docs/issues/win-corner-br2.png) |

-   **issue(floem)**: Il n'y a pour l'instant pas la possibilité d'arrondir
    uniquement certaines parties d'un élément.

Exemple, il n'y a pas les propriétés suivantes:

1. `border_top_left_radius`, `border_top_right_radius`
2. `border_bottom_left_radius` `border_bottom_right_radius`
3. `border_top_radius` (top left & top right)
4. `border_bottom_radius` (bottom left & bottom right)
5. `border_left_radius` (top left & bottom left)
6. `border_right_radius` (top right & bottom right)

-   **issue(floem)**: pas trouver le moyen de styliser correctement un SVG.

-   **issue(floem)**: La propriété `Style#transition` sur les couleurs de
    premier plan ne semble pas fonctionner sur les éléments de types SVG.

Exemple:

```rs
parent_view.style(|s| {
	s.class(IconWithOpacity, |s| {
		s
			// ...
			// ... .color(...)
			// ...
			.transition(
				style::TextColor,
				style::Transition::linear(2.0),
			)
			.transition(
				style::Foreground,
				style::Transition::linear(2.0),
			)
	})
});

svg_element.class(IconWithOpacity);
```

-   **issue(std / stream)**: pas trouver le moyen de cloner un stream, buffer,
    d'un fichier ou d'une réponse de requête, pour éventuellement afficher la
    partie brut HTML côté frontend
