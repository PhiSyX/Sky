# Notes

-   **info(sky)**: les commentaires `// dfplz`, `// don't format please` et
    autre variantes sont uniquement présents pour que le formater de rust
    n'effectuent pas de transformation sur les parties du code en question.
    N'est pas une règle officielle du formater.

-   **info(sky)**: notre parser HTML n'est pas encore utilisable, on utilise
    pour l'instant `html5ever`.

-   **issue(floem)**: La fenêtre prend en considération la propriété de style
    `Style.border_radius(N)`, mais l'arrière-plan n'est pas transparent. Ce qui
    pose un souci, `WindowConfig.with_transparent(true)` ne fait qu'empirer le
    souci.

    | WindowConfig.with_transparent(false)       | WindowConfig.with_transparent(true)        |
    | ------------------------------------------ | ------------------------------------------ |
    | ![Fenêtre](docs/issues/win-corner-br1.png) | ![Fenêtre](docs/issues/win-corner-br2.png) |

-   **issue(floem)**: pas trouver le moyen de styliser correctement un SVG.

-   **issue(stream)**: pas trouver le moyen de cloner un stream, d'un fichier ou
    d'une réponse de requête, pour éventuellement afficher la partie brut HTML
    côté frontend
