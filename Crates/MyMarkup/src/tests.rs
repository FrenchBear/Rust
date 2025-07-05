// tests.rs - MyMarkup tests
//
// 2025-07-05   PV      First version with MyMarkup 1.1 that can generate string output

#![cfg(test)]

use crate::*;

#[test]
fn test_30() {
    let text = "Ceci est un texte à formater, avec un mot très long comme anticonstitutionnellement qui va être tronqué quand la largeur du rendu devient particulièrement petite.";

    let expected = "------------------------------
Ceci est un texte à formater, |
avec un mot très long comme   |
anticonstitutionnellement qui |
va être tronqué quand la      |
largeur du rendu devient      |
particulièrement petite.      |";

    let s = MyMarkup::build_markup_core(text, true, 30);
    assert_eq!(s, expected);
}

#[test]
fn test_25() {
    let text = "Ceci est un texte à formater, avec un mot très long comme anticonstitutionnellement qui va être tronqué quand la largeur du rendu devient particulièrement petite.";

    let expected = "-------------------------
Ceci est un texte à      |
formater, avec un mot    |
très long comme          |
anticonstitutionnellement|
qui va être tronqué quand|
la largeur du rendu      |
devient particulièrement |
petite.                  |";

    let s = MyMarkup::build_markup_core(text, true, 25);
    assert_eq!(s, expected);
}

#[test]
fn test_20() {
    let text = "Ceci est un texte à formater, avec un mot très long comme anticonstitutionnellement qui va être tronqué quand la largeur du rendu devient particulièrement petite.";

    let expected = "--------------------
Ceci est un texte à |
formater, avec un   |
mot très long comme |
anticonstitutionnell|
ement qui va être   |
tronqué quand la    |
largeur du rendu    |
devient             |
particulièrement    |
petite.             |";

    let s = MyMarkup::build_markup_core(text, true, 20);
    assert_eq!(s, expected);
}

#[test]
fn test_15() {
    let text = "Ceci est un texte à formater, avec un mot très long comme anticonstitutionnellement qui va être tronqué quand la largeur du rendu devient particulièrement petite.";

    let expected = "---------------
Ceci est un    |
texte à        |
formater, avec |
un mot très    |
long comme     |
anticonstitutio|
nnellement qui |
va être tronqué|
quand la       |
largeur du     |
rendu devient  |
particulièremen|
t petite.      |";

    let s = MyMarkup::build_markup_core(text, true, 15);
    assert_eq!(s, expected);
}

#[test]
fn test_10() {
    let text = "Ceci est un texte à formater, avec un mot très long comme anticonstitutionnellement qui va être tronqué quand la largeur du rendu devient particulièrement petite.";

    let expected = "----------
Ceci est  |
un texte à|
formater, |
avec un   |
mot très  |
long comme|
anticonsti|
tutionnell|
ement qui |
va être   |
tronqué   |
quand la  |
largeur du|
rendu     |
devient   |
particuliè|
rement    |
petite.   |";

    let s = MyMarkup::build_markup_core(text, true, 10);
    assert_eq!(s, expected);
}
