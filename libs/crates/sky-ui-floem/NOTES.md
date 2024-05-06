# Notes

-   **issue**: La fenêtre prend en considération la propriété de style
    `Style.border_radius(N)`, mais l'arrière-plan n'est pas transparent. Ce qui
    pose un souci, `WindowConfig.with_transparent(true)` ne fait qu'empirer le
    souci.

    | WindowConfig.with_transparent(false)       | WindowConfig.with_transparent(true)        |
    | ------------------------------------------ | ------------------------------------------ |
    | ![Fenêtre](docs/issues/win-corner-br1.png) | ![Fenêtre](docs/issues/win-corner-br2.png) |

-   **issue**: pas trouver le moyen de styliser correctement un SVG.
