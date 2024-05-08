# Mini navigateur

Work In Progress, il s'agit d'un projet expérimental.
Tout n'est pas fonctionnel. Tout n'est pas propre.

Calme-toi! L'idée n'est pas de concurrencer Firefox, Chrome, Brave, Edge, Arc,
etc.

## UI

Capture d'écran actuel de l'app:

![UI](docs/screenshots/ui-current.png?v0.1.0)

## Notes

-   **known-issue**: lenteur en mode debug sur des gros jeux de données. Cela
	est dû au fait que nous lisons d'une traite le buffer pour afficher le
	contenu en brut dans l'application. C'est volontaire, pour le moment.

-   **info(sky)**: les commentaires `// dfplz`, `// don't format please` et
    autre variantes sont uniquement présents pour que le formater de rust
    n'effectuent pas de transformation sur les parties du code en question.
    N'est pas une règle officielle du formater.

-   **info(sky)**: notre parser HTML n'est pas encore utilisable, on utilise
    pour l'instant `html5ever`.

-   **praise(floem)**: simple d'utilisation, API claire, utilisation des signaux
    pour le côté réactif.

-   **praise(floem)**: nous pouvons étendre les classes déjà existantes, bon
    point.

-   **issue(floem)**: n'est pour l'instant pas possible d'`prepend`/`append` des
    éléments sur un container d'élément.

Exemple:

```rs
let contents = make_contents(); // type: Iterator<Item = T> / Vec<T> / ...
let footer = make_footer(); // impl View / AnyView / Container / ...

let stack = v_stack(( header, subheader ))
	.append_iter( contents )
	.append( footer )
;
```

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

-   **issue(floem)**: La propriété `Style#background` à la valeur
    `Color::TRANSPARENT` n'est pas pris en compte pour tous état possible
	hover, focus, etc... On se retrouve à faire des choses moche comme suit:

```rs
text_input(...).style(|s| {
	s.active(|s| s.background(Color::TRANSPARENT))
	.hover(|s| s.background(Color::TRANSPARENT))
	.focus(|s| {
		s
			.background(Color::TRANSPARENT)
			.hover(|s| s.background(Color::TRANSPARENT))
	})
	.selected(|s| s.background(Color::TRANSPARENT))
	.focus_visible(|s| s.background(Color::TRANSPARENT))
	.background(Color::TRANSPARENT)
})
```

-   **issue(std / stream)**: pas trouver le moyen de copier un
    stream/buffer, d'un fichier ou d'une réponse de requête de manière
    efficiente, pour éventuellement afficher la partie brut HTML côté
    frontend en debug.
