// tests_transform.rs
// Test global transformation
//
// 2025-04-14   PV
// 2025-04-16   PV      Better normalization of n°

#![cfg(test)]

use crate::*;

fn t(dp: &DatePatterns, before: &str, expected: &str) {
    let mut stem = apply_initial_transformations(&before);
    stem = apply_date_transformations(&stem, dp, false);
    stem = apply_final_transformations(&stem);

    assert_eq!(expected, stem);
}

#[test]
fn tests_transformations() {
    let dp = DatePatterns::new();

    t(&dp, "L’Informaticien - Avril 2025" ,"L'Informaticien - 2025-04");
    t(&dp, "Ça M’Intéresse Questions & Réponses - Avril-Juin 2025" ,"Ça m'intéresse Questions Réponses - 2025-04..06");
    t(&dp, "JV Le Mag #114 - Mars 2025" ,"JV Le Mag #114 - 2025-03");
    t(&dp, "Micro Pratique - 05-06.2025" ,"Micro Pratique - 2025-05..06");
    t(&dp, "L’Auto-Journal - Le Guide N°66 - Avril-Juin 2025" ,"Le guide de l'Auto-Journal n°66 - 2025-04..06");
    t(&dp, "Auto Plus - 11 Avril 2025" ,"Auto Plus - 2025-04-11");
    t(&dp, "Détours en France - 05.2025" ,"Détours en France - 2025-05");
    t(&dp, "Geo France - HS - 04-05.2025" ,"Géo - HS - 2025-04..05");
    t(&dp, "Destination Europe - 05-06-07.2025" ,"Destination Europe - 2025-05..07");
    t(&dp, "01net du 09.04.2025" ,"01net - 2025-04-09");
    t(&dp, "Science & Vie - Guerres & Histoire - 05.2025" ,"Science & Vie Guerres & Histoire - 2025-05");
    t(&dp, "Réponses Photo - 05.2025" ,"Réponses Photo - 2025-05");
    t(&dp, "Charlie Hebdo - 9 Avril 2025" ,"Charlie Hebdo - 2025-04-09");
    t(&dp, "Canard PC - 465 - Avril 2025" ,"Canard PC - 465 - 2025-04");
    t(&dp, "Le Canard enchaîné - 2025-04-09 - N°5448, 09 Avril 2025" ,"Le canard enchainé - 2025-04-09 - n°5448, 09 Avril 2025");
    t(&dp, "Canard PC - 462 - Janvier 2025" ,"Canard PC - 462 - 2025-01");
    t(&dp, "Canard PC - 464 - Mars 2025" ,"Canard PC - 464 - 2025-03");
    t(&dp, "Canard PC - 463 - Fevrier 2025" ,"Canard PC - 463 - 2025-02");
    t(&dp, "BBC Science Focus 2024 №413 December" ,"BBC Science Focus n°413 - 2024-12");
    t(&dp, "Cerveau___Psycho_-_Mai_2020" ,"Cerveau & Psycho - 2020-05");
    t(&dp, "BBC Wildlife 2024 №2 February 2025" ,"BBC Wildlife n°2 - 2024-02 - 2025");
    t(&dp, "Windows Internet Pratique - 02-03.2025" ,"Windows & Internet Pratique - 2025-02..03");
    t(&dp, "Pirate Informatique - 02-03-04.2025" ,"Pirate Informatique - 2025-02..04");
    t(&dp, "60 Millions de Consommateurs - HS - 02-03.2025" ,"60M de consommateurs - HS - 2025-02..03");
    t(&dp, "01net du 22.01.2025" ,"01net - 2025-01-22");
    t(&dp, "01Net Hors-Série - N°143 - 03-04.2025" ,"01net HS - n°143 - 2025-03..04");
    t(&dp, "Science & Vie - 02.2025" ,"Science & Vie - 2025-02");
    t(&dp, "60 Millions de Consommateurs - 02.2025" ,"60M de consommateurs - 2025-02");
    t(&dp, "01Net Hors-Série - N°142 - 01-02.2025" ,"01net HS - n°142 - 2025-01..02");
    t(&dp, "60 Millions de Consommateurs - HS - 01-02.2025" ,"60M de consommateurs - HS - 2025-01..02");
    t(&dp, "Auto Plus du 17.01.2025" ,"Auto Plus - 2025-01-17");
    t(&dp, "BBC Science Focus 2025 №415 January" ,"BBC Science Focus n°415 - 2025-01");
    t(&dp, "01net du 08.01.2025" ,"01net - 2025-01-08");
    t(&dp, "L'Histoire - 02.2025" ,"L'Histoire - 2025-02");
    t(&dp, "Auto Plus du 07.02.2025" ,"Auto Plus - 2025-02-07");
    t(&dp, "Science & Vie - 01.2025" ,"Science & Vie - 2025-01");
    t(&dp, "60 Millions de Consommateurs - HS - 12.2024" ,"60M de consommateurs - HS - 2024-12");
    t(&dp, "Sciences et Avenir - 02.2025" ,"Sciences et Avenir - 2025-02");
    t(&dp, "Que Choisir - Novembre 2024" ,"Que Choisir - 2024-11");
    t(&dp, "Elektor n°511 2025-01-02" ,"Elektor n°511 2025-01-02");
    t(&dp, "01net du 05.02.2025" ,"01net - 2025-02-05");
    t(&dp, "Pour la Science - 02.2025" ,"Pour la Science - 2025-02");
    t(&dp, "Que choisir - 01.2025" ,"Que choisir - 2025-01");
    t(&dp, "60 Millions de Consommateurs - 01.2025" ,"60M de consommateurs - 2025-01");
    t(&dp, "60 Millions de consommateurs - HS - 11-12.2024" ,"60M de consommateurs - HS - 2024-11..12");
    t(&dp, "60 Millions de Consommateurs - 12.2024" ,"60M de consommateurs - 2024-12");
    t(&dp, "Windows Internet Pratique - 01-02.2025" ,"Windows & Internet Pratique - 2025-01..02");
    t(&dp, "What Hi-Fi - 02.2025" ,"What Hi-Fi - 2025-02");
    t(&dp, "Auto Plus - HS - Crossovers-Suv - 02-03-04.2025" ,"Auto Plus Crossovers - 2025-02..04");
    t(&dp, "Canard PC Hardware - 01-02.2025" ,"Canard PC Hardware - 2025-01..02");
    t(&dp, "Sciences et Avenir - HS - 01-02-03.2025" ,"Sciences et Avenir - HS - 2025-01..03");
    t(&dp, "Micro Pratique - 03-04.2025" ,"Micro Pratique - 2025-03..04");
    t(&dp, "Réponses Photo - 03.2025" ,"Réponses Photo - 2025-03");
    t(&dp, "Ca m'intéresse - 01.2025" ,"Ca m'intéresse - 2025-01");
    t(&dp, "Micro pratique - 02-03.2025" ,"Micro pratique - 2025-02..03");
    t(&dp, "PC trucs et astuces - 12.2024 01-02.2025" ,"PC trucs et astuces - 2024-12..2025-02");
    t(&dp, "PC trucs et astuces - 11-12.2024 01.2025" ,"PC trucs et astuces - 2024-11..2025-01");
    t(&dp, "Que choisir - 02.2025" ,"Que choisir - 2025-02");
    t(&dp, "Auto Plus du 31.01.2025" ,"Auto Plus - 2025-01-31");
    t(&dp, "60 Millions de Consommateurs - 11.2024" ,"60M de consommateurs - 2024-11");
    t(&dp, "Hackable Magazine - Janvier-Février 2025" ,"Hackable Magazine - 2025-01..02");
    t(&dp, "Auto Plus - 10 Janvier 2025" ,"Auto Plus - 2025-01-10");
    t(&dp, "Auto Plus du 24.01.2025" ,"Auto Plus - 2025-01-24");
    t(&dp, "Que Choisir Santé - 02.2025" ,"Que Choisir Santé - 2025-02");
    t(&dp, "What Hi-Fi - 11.2024" ,"What Hi-Fi - 2024-11");
    t(&dp, "Que choisir Pratique - 12.2024" ,"Que choisir Pratique - 2024-12");
    t(&dp, "Auto Plus du 14.02 au 27.02.2025" ,"Auto Plus - 2025-02-14..02-27");
    t(&dp, "Historia_-_F_vrier_2025" ,"Historia - 2025-02");
    t(&dp, "Hackable Magazine - Novembre-Décembre 2024" ,"Hackable Magazine - 2024-11..12");
    t(&dp, "Cerveau___Psycho_-_F_vrier_2025" ,"Cerveau & Psycho - 2025-02");
    t(&dp, "_a_M_Int_resse_Questions___R_ponses_-_Janvier-Mars_2025" ,"Ça m'intéresse Questions Réponses - 2025-01..03");
    t(&dp, "National Geographic - Hors-Série - 02-03.2025" ,"National Geographic - HS - 2025-02..03");
    t(&dp, "L'Automobile Magazine - 02.2025" ,"L'Automobile Magazine - 2025-02");
    t(&dp, "Que Choisir Santé - 01.2025" ,"Que Choisir Santé - 2025-01");
    t(&dp, "L’Informaticien - 02.2025" ,"L'Informaticien - 2025-02");
    t(&dp, "Science & Vie - Guerres & Histoire - 03.2025" ,"Science & Vie Guerres & Histoire - 2025-03");
    t(&dp, "Les Dossiers du Pirate - 01-02-03.2025" ,"Les Dossiers du Pirate - 2025-01..03");
    t(&dp, "Que choisir - HS Budgets - 01.2025" ,"Que choisir Budgets - 2025-01");
    t(&dp, "Destination USA - 03-04-05.2025" ,"Destination USA - 2025-03..05");
    t(&dp, "Les Cahiers de Science & Vie - 03-04.2025" ,"Les Cahiers de Science & Vie - 2025-03..04");
    t(&dp, "Cerveau___Psycho_-_Juin_2022" ,"Cerveau & Psycho - 2022-06");
    t(&dp, "Epsiloon - HS - 01-02-03.2025" ,"Epsiloon - HS - 2025-01..03");
    t(&dp, "Que Choisir - Octobre 2024" ,"Que Choisir - 2024-10");
    t(&dp, "T3 France_Février 2025" ,"T3 - 2025-02");
    t(&dp, "Cerveau & Psycho - Novembre 2017" ,"Cerveau & Psycho - 2017-11");
    t(&dp, "Cerveau___Psycho_-_Juin_2023" ,"Cerveau & Psycho - 2023-06");
    t(&dp, "Destination Europe - 01-02-03.2025" ,"Destination Europe - 2025-01..03");
    t(&dp, "Cerveau___Psycho_-_Avril_2023" ,"Cerveau & Psycho - 2023-04");
    t(&dp, "Cerveau__Psycho__Septembre_2017" ,"Cerveau & Psycho - 2017-09");
    t(&dp, "Epsiloon - 02.2025" ,"Epsiloon - 2025-02");
    t(&dp, "Cerveau  Psycho  Octobre 2017" ,"Cerveau & Psycho - 2017-10");
    t(&dp, "Moto Revue - 03.2025" ,"Moto Revue - 2025-03");
    t(&dp, "Le Figaro Histoire - 02-03.2025" ,"Le Figaro Histoire - 2025-02..03");
    t(&dp, "Le Canard enchaîné - 2025-02-05 - N°5439, 05 Février 2025" ,"Le canard enchainé - 2025-02-05 - n°5439, 05 Février 2025");
    t(&dp, "National Geographic 2024 №11 November" ,"National Geographic n°11 - 2024-11");
    t(&dp, "Charlie Hebdo du 22.01.2025" ,"Charlie Hebdo - 2025-01-22");
    t(&dp, "Cerveau & Psycho - Janvier 2025" ,"Cerveau & Psycho - 2025-01");
    t(&dp, "Cerveau & Psycho - December 2018" ,"Cerveau & Psycho - 2018-12");
    t(&dp, "National Geographic 2024 №12 December" ,"National Geographic n°12 - 2024-12");
    t(&dp, "Terre Sauvage - 02.2025" ,"Terre Sauvage - 2025-02");
    t(&dp, "Cerveau & Psycho Spécial Numéro 100 - Juin 2018" ,"Cerveau & Psycho Spécial Numéro 100 - 2018-06");
    t(&dp, "National Geographic 2025 №01 January" ,"National Geographic n°01 - 2025-01");
    t(&dp, "Charlie_Hebdo_-_1er_Janvier_2025" ,"Charlie Hebdo - 2025-01-01");
    t(&dp, "Le_Journal_de_Spirou_-_8_Janvier_2025" ,"Le Journal de Spirou - 2025-01-08");
    t(&dp, "National Geographic 2025 №02 February" ,"National Geographic n°02 - 2025-02");
    t(&dp, "Cerveau___Psycho_-_Mai_2023" ,"Cerveau & Psycho - 2023-05");
    t(&dp, "Cerveau___Psycho_-_Novembre_2020" ,"Cerveau & Psycho - 2020-11");
    t(&dp, "BBC Wildlife 2024 №13 December" ,"BBC Wildlife n°13 - 2024-12");
    t(&dp, "Moto.Magazine.N.416.Fevrier.2025.francais.PDF-Notag" ,"Moto Magazine n°416 - 2025-02");
    t(&dp, "Cerveau___Psycho_-_Octobre_2023" ,"Cerveau & Psycho - 2023-10");
    t(&dp, "Cerveau___Psycho_-_Mars_2023" ,"Cerveau & Psycho - 2023-03");
    t(&dp, "Cerveau_Psycho - Septembre 2018" ,"Cerveau & Psycho - 2018-09");
    t(&dp, "Le.journal.de.Mickey.N.3784-85.24.decembre.2024.FRENCH.PDF-Notag" ,"Le journal de Mickey n°3784-85 - 2024-12-24");
    t(&dp, "Cerveau___Psycho_-_Octobre_2020b" ,"Cerveau & Psycho - 2020-10");
    t(&dp, "Cerveau___Psycho_-_Avril_2022" ,"Cerveau & Psycho - 2022-04");
    t(&dp, "Cerveau___Psycho_-_Juillet-Ao_t_2023" ,"Cerveau & Psycho - 2023-07..08");
    t(&dp, "Cerveau & Psycho N°160 Décembre 2023" ,"Cerveau & Psycho n°160 - 2023-12");
    t(&dp, "Cerveau___Psycho_-_Septembre_2022" ,"Cerveau & Psycho - 2022-09");
    t(&dp, "Charlie_Hebdo_-_29_Janvier_2025" ,"Charlie Hebdo - 2025-01-29");
    t(&dp, "Cerveau Psycho - Avril 2020B" ,"Cerveau & Psycho - 2020-04");
    t(&dp, "Cerveau & Psycho Ndeg124 - septembre 2020" ,"Cerveau & Psycho Ndeg124 - 2020-09");
    t(&dp, "Charlie_Hebdo_-_5_F_vrier_2025" ,"Charlie Hebdo - 2025-02-05");
    t(&dp, "Cerveau___Psycho_-_Janvier_2023" ,"Cerveau & Psycho - 2023-01");
    t(&dp, "Cerveau___Psycho_-_D_cembre_2022" ,"Cerveau & Psycho - 2022-12");
    t(&dp, "Echappée Belle - 01.2025" ,"Échappée Belle - 2025-01");
    t(&dp, "Moto Verte - 02-03.2025" ,"Moto Verte - 2025-02..03");
    t(&dp, "Magazine CERVEAU et PSYCHO N.10 - Fevrier 2019" ,"Cerveau & Psycho n°10 - 2019-02");
    t(&dp, "Cerveau___Psycho_-_Septembre_2023" ,"Cerveau & Psycho - 2023-09");
    t(&dp, "Grands Reportages - 02-03.2025" ,"Grands Reportages - 2025-02..03");
    t(&dp, "Cerveau___Psycho_-_Mai_2022" ,"Cerveau & Psycho - 2022-05");
    t(&dp, "Cerveau___Psycho_-_Novembre_2023" ,"Cerveau & Psycho - 2023-11");
    t(&dp, "Le Canard enchaîné - 2024-11-06 - N°5426, 06 Novembre 2024" ,"Le canard enchainé - 2024-11-06 - n°5426, 06 Novembre 2024");
    t(&dp, "Cerveau___Psycho_-_Fevrier_2022" ,"Cerveau & Psycho - 2022-02");
    t(&dp, "Cerveau & Psycho ndeg108 - Mars 2019" ,"Cerveau & Psycho ndeg108 - 2019-03");
    t(&dp, "Cerveau___Psycho_-_Octobre_2022" ,"Cerveau & Psycho - 2022-10");
    t(&dp, "Cerveau___Psycho_-_Juillet-Ao_t_2022" ,"Cerveau & Psycho - 2022-07..08");
    t(&dp, "Cerveau___Psycho_-_D_cembre_2020b" ,"Cerveau & Psycho - 2020-12");
    t(&dp, "Cerveau___Psycho_-_F_vrier_2023" ,"Cerveau & Psycho - 2023-02");
    t(&dp, "Cerveau___Psycho-Novembre_2022" ,"Cerveau & Psycho - 2022-11");
    t(&dp, "Le Canard enchaîné - 2025-01-15 - N°5436, 15 Janvier 2025" ,"Le canard enchainé - 2025-01-15 - n°5436, 15 Janvier 2025");
    t(&dp, "Le Canard enchaîné - 2024-10-30 - N°5425, 30 Octobre 2024" ,"Le canard enchainé - 2024-10-30 - n°5425, 30 Octobre 2024");
    t(&dp, "Le Canard enchaîné - 2025-01-08 - N°5435, 08 Janvier 2025" ,"Le canard enchainé - 2025-01-08 - n°5435, 08 Janvier 2025");
    t(&dp, "Cerveau___Psycho_-_Janvier_2022" ,"Cerveau & Psycho - 2022-01");
    t(&dp, "CERVEAU & PSYCHO Ndeg119 - MARS 2020" ,"Cerveau & Psycho Ndeg119 - 2020-03");
    t(&dp, "Cerveau___Psycho_-_Juin_2020" ,"Cerveau & Psycho - 2020-06");
    t(&dp, "Cerveau___Psycho_-_Octobre_2018" ,"Cerveau & Psycho - 2018-10");
    t(&dp, "Moto Journal - 02.2025" ,"Moto Journal - 2025-02");
    t(&dp, "Auto Moto - 02.2025" ,"Auto Moto - 2025-02");
    t(&dp, "Paris Match - HS - N°49 - 02.2025" ,"Paris Match - HS - n°49 - 2025-02");
    t(&dp, "Désirs de Voyages - N.92 - 2025" ,"Désirs de Voyages - n°92 - 2025");
    t(&dp, "What Hi-Fi France - Janvier 2025" ,"What Hi-Fi - 2025-01");
    t(&dp, "VTT Magazine - 02-03.2025" ,"VTT Magazine - 2025-02..03");
    t(&dp, "Cerveau___Psycho_-_Avril_2018" ,"Cerveau & Psycho - 2018-04");
    t(&dp, "Cerveau & Psycho No.95 (Janvier  2018)" ,"Cerveau & Psycho n°95 (2018-01)");
    t(&dp, "Que Choisir - 12.2024" ,"Que Choisir - 2024-12");
    t(&dp, "Cerveau_Psycho_2020_01" ,"Cerveau & Psycho - 2020-01");
    t(&dp, "Cerveau___Psycho_-_Juillet-Ao_t_2020" ,"Cerveau & Psycho - 2020-07..08");
    t(&dp, "Moto Revue - Hors-Série Spécial Essais - 2025" ,"Moto Revue - HS Spécial Essais - 2025");
    t(&dp, "Pour la Science - HS - 05-06.2025" ,"Pour la Science - HS - 2025-05..06");
    t(&dp, "L’Histoire - HS N.107 - 04-05-06.2025" ,"L'Histoire - HS n°107 - 2025-04..06");
    t(&dp, "Sciences et Avenir - HS - 04-05-06.2025" ,"Sciences et Avenir - HS - 2025-04..06");
    t(&dp, "Auto Plus du 04.04.2025" ,"Auto Plus - 2025-04-04");
    t(&dp, "Les Cahiers de Science & Vie - 05-06.2025" ,"Les Cahiers de Science & Vie - 2025-05..06");
    t(&dp, "Que Choisir Hors-Série Budgets - 03.2025" ,"Que Choisir HS Budgets - 2025-03");
    t(&dp, "Auto Moto - 04.2025" ,"Auto Moto - 2025-04");
    t(&dp, "Charlie Hebdo - 2 Avril 2025" ,"Charlie Hebdo - 2025-04-02");
    t(&dp, "Que Choisir Santé - 04.2025" ,"Que Choisir Santé - 2025-04");
    t(&dp, "Charlie Hebdo - 26 Mars 2025" ,"Charlie Hebdo - 2025-03-26");
    t(&dp, "Epsiloon - 04.2025" ,"Epsiloon - 2025-04");
    t(&dp, "Science & Vie - Hors-Série - 05.2025" ,"Science & Vie - HS - 2025-05");
    t(&dp, "Que Choisir - 04.2025" ,"Que Choisir - 2025-04");
    t(&dp, "National Geographic 2025 №04 April" ,"National Geographic n°04 - 2025-04");
    t(&dp, "Pour la Science N°570 - Avril 2025" ,"Pour la Science n°570 - 2025-04");
    t(&dp, "Échappée Belle - 03.2025" ,"Échappée Belle - 2025-03");
    t(&dp, "Auto Plus du 28.03.2025" ,"Auto Plus - 2025-03-28");
    t(&dp, "Geo France - 04.2025" ,"Géo - 2025-04");
    t(&dp, "Détours en France - HS - Printemps 2025" ,"Détours en France - HS - 2025-Printemps");
    t(&dp, "National Geographic - 04.2025" ,"National Geographic - 2025-04");
    t(&dp, "Grands Reportages - 04-05.2025" ,"Grands Reportages - 2025-04..05");
    t(&dp, "L’Histoire - 04.2025" ,"L'Histoire - 2025-04");
    t(&dp, "Sciences et Avenir - 04.2025" ,"Sciences et Avenir - 2025-04");
    t(&dp, "Le Figaro Histoire - 04-05.2025" ,"Le Figaro Histoire - 2025-04..05");
    t(&dp, "60 Millions de Consommateurs - 04.2025" ,"60M de consommateurs - 2025-04");
    t(&dp, "Science & Vie - 04.2025" ,"Science & Vie - 2025-04");
    t(&dp, "01net du 26.03.2025" ,"01net - 2025-03-26");
    t(&dp, "BBC Wildlife 2025 №04 Spring" ,"BBC Wildlife n°04 - 2025-Printemps");
    t(&dp, "BBC Science Focus 2025 №417 March" ,"BBC Science Focus n°417 - 2025-03");
    t(&dp, "Historia - 04.2025" ,"Historia - 2025-04");
    t(&dp, "Cerveau & Psycho - Avril 2025" ,"Cerveau & Psycho - 2025-04");
    t(&dp, "Moto Journal - 04.2025" ,"Moto Journal - 2025-04");
    t(&dp, "Geo Histoire - 03-04.2025" ,"Géo Histoire - 2025-03..04");
    t(&dp, "L'Auto-Journal 4x4 - 04-05-06.2025" ,"L'Auto-Journal 4x4 - 2025-04..06");
    t(&dp, "Auto Plus - Guide de L’Acheteur - 04-05-06.2025" ,"Auto Plus Guide de l'acheteur - 2025-04..06");
    t(&dp, "Destination Grèce - 04-05-06.2025" ,"Destination Grèce - 2025-04..06");
    t(&dp, "Ca m'intéresse - 04.2025" ,"Ca m'intéresse - 2025-04");
    t(&dp, "Terre Sauvage - 04.2025" ,"Terre Sauvage - 2025-04");
    t(&dp, "Hackable_2025_03_04" ,"Hackable - 2025-03-04 $$$ 2025-03..04");
    t(&dp, "Le Monde - Histoire & Civilisations - 04.2025" ,"Histoire & Civilisations - 2025-04");
    t(&dp, "Auto Plus du 21.03.2025" ,"Auto Plus - 2025-03-21");
    t(&dp, "ON Magazine - N.1 - 2025" ,"ON Magazine - n°1 - 2025");
    t(&dp, "L’Essentiel de l’Auto - 04-05-06.2025" ,"L'essentiel de l'Auto - 2025-04..06");
    t(&dp, "Spécial Histoire - 03-04-05.2025" ,"Spécial Histoire - 2025-03..05");
    t(&dp, "L’Auto-Journal du 20.03.2025" ,"L'Auto-Journal - 2025-03-20");
    t(&dp, "Science [Vol.387 #6733] - 2025.01.31" ,"Science [Vol 387 #6733] - 2025-01-31");
    t(&dp, "L’Informaticien - 03.2025" ,"L'Informaticien - 2025-03");
    t(&dp, "Auto Plus Vert - 04-05-06.2025" ,"Auto Plus Vert - 2025-04..06");
    t(&dp, "Moto Revue - 04.2025" ,"Moto Revue - 2025-04");
    t(&dp, "Pirate Informatique.Novembre.2024.Janvier.2025.N°62" ,"Pirate Informatique - 2024-11..2025-01 - n°62");
    t(&dp, "T3 France - 03.2025" ,"T3 - 2025-03");
    t(&dp, "Science [Vol.387 #6729] - 2025.01.03" ,"Science [Vol 387 #6729] - 2025-01-03 $$$ 2025-01..03");
    t(&dp, "Science [Vol.387 #6730] - 2025.01.10" ,"Science [Vol 387 #6730] - 2025-01-10 $$$ 2025-01..10");
    t(&dp, "Science [Vol.387 #6732] - 2025.01.24" ,"Science [Vol 387 #6732] - 2025 - 2024-01");
    t(&dp, "Science [Vol.387 #6731] - 2025.01.17" ,"Science [Vol 387 #6731] - 2025-01-17");
    t(&dp, "Moto Verte - 04-05.2025" ,"Moto Verte - 2025-04..05");
    t(&dp, "Charlie Hebdo - 12 Mars 2025" ,"Charlie Hebdo - 2025-03-12");
    t(&dp, "Moto Magazine - 04.2025" ,"Moto Magazine - 2025-04");
    t(&dp, "01net du 12.03.2025" ,"01net - 2025-03-12");
    t(&dp, "Auto Plus du 14.03.2025" ,"Auto Plus - 2025-03-14");
    t(&dp, "Les Dossiers du Pirate - 10-11-12.2024" ,"Les Dossiers du Pirate - 2024-10..12");
    t(&dp, "Les Dossiers du Pirate - 07-08-09.2024" ,"Les Dossiers du Pirate - 2024-07..09");
    t(&dp, "What Hi-Fi - 03.2025" ,"What Hi-Fi - 2025-03");
    t(&dp, "L'Automobile Magazine - 03.2025" ,"L'Automobile Magazine - 2025-03");
    t(&dp, "Micro Pratique - 04-05.2025" ,"Micro Pratique - 2025-04..05");
    t(&dp, "Réponses Photo - 04.2025" ,"Réponses Photo - 2025-04");
    t(&dp, "Auto Plus du 07.03.2025" ,"Auto Plus - 2025-03-07");
    t(&dp, "60 Millions de Consommateurs - HS - 04-05.2025" ,"60M de consommateurs - HS - 2025-04..05");
    t(&dp, "Détours en France - 04.2025" ,"Détours en France - 2025-04");
    t(&dp, "Moto.Revue.Hors-Série.MotoGP.2025" ,"Moto Revue HS MotoGP 2025");
    t(&dp, "Moto Magazine - HS - 03-04-05.2025" ,"Moto Magazine - HS - 2025-03..05");
    t(&dp, "Moto Magazine - 03.2025" ,"Moto Magazine - 2025-03");
    t(&dp, "Destination France - 03.04.05.2025" ,"Destination France - 2025-03..05");
    t(&dp, "Direction Canada - 03-04.05.2025" ,"Direction Canada - 2025-03..05");
    t(&dp, "Charlie Hebdo - 20 Février 2025" ,"Charlie Hebdo - 2025-02-20");
    t(&dp, "L’Histoire - 03.2025" ,"L'Histoire - 2025-03");
    t(&dp, "Historia - Hors-Série - 03.2025" ,"Historia - HS - 2025-03");
    t(&dp, "Charlie Hebdo - 26 Février 2025" ,"Charlie Hebdo - 2025-02-26");
    t(&dp, "Auto Plus du 28.02.2025" ,"Auto Plus - 2025-02-28");
    t(&dp, "Que.Choisir.Sante.N.202.Mars.2025.FRENCH.PDF-Notag" ,"Que Choisir Santé n°202 - 2025-03");
    t(&dp, "Pour la Science - 03.2025" ,"Pour la Science - 2025-03");
    t(&dp, "BBC Science Focus 2025 №416 February" ,"BBC Science Focus n°416 - 2025-02");
    t(&dp, "National Geographic - 03.2025" ,"National Geographic - 2025-03");
    t(&dp, "Moto Revue - HS Ed.2 - Salon 2025 - 1300 motos" ,"Moto Revue - HS Ed 2 - Salon 2025 - 1300 motos");
    t(&dp, "Geo France - 03.2025" ,"Géo - 2025-03");
    t(&dp, "Epsiloon - 03.2025" ,"Epsiloon - 2025-03");
    t(&dp, "Sciences et Avenir - 03.2025" ,"Sciences et Avenir - 2025-03");
    t(&dp, "BBC Wildlife 2025 №03 March" ,"BBC Wildlife n°03 - 2025-03");
    t(&dp, "60 Millions de Consommateurs - 03.2025" ,"60M de consommateurs - 2025-03");
    t(&dp, "National Geographic 2025 №03 March" ,"National Geographic n°03 - 2025-03");
    t(&dp, "Échappée Belle - 02.2025" ,"Échappée Belle - 2025-02");
    t(&dp, "Science & Vie - 03.2025" ,"Science & Vie - 2025-03");
    t(&dp, "Stop Arnaques - 03-04-05.2025" ,"Stop Arnaques - 2025-03..05");
    t(&dp, "Jeux Vidéo Magazine - 03.2025" ,"Jeux Vidéo Magazine - 2025-03");
    t(&dp, "PC trucs et astuces - 03-04-05.2025" ,"PC trucs et astuces - 2025-03..05");
    t(&dp, "Terre Sauvage - 03.2025" ,"Terre Sauvage - 2025-03");
    t(&dp, "Moto Journal - 03.2025" ,"Moto Journal - 2025-03");
    t(&dp, "Historia - 03.2025" ,"Historia - 2025-03");
    t(&dp, "L'essentiel de la science - 03-04-05.2025" ,"L'essentiel de la science - 2025-03..05");
    t(&dp, "L’Auto-Journal du 20.02.2025" ,"L'Auto-Journal - 2025-02-20");
    t(&dp, "Grands Reportages - 03-04-05.2025" ,"Grands Reportages - 2025-03..05");
    t(&dp, "Ça M'Intéresse - 03.2025" ,"Ça m'intéresse - 2025-03");
    t(&dp, "Le Monde - Histoire & Civilisations - 03.2025" ,"Histoire & Civilisations - 2025-03");
    t(&dp, "01net du 19.02.2025" ,"01net - 2025-02-19");
    t(&dp, "Que choisir - 03.2025" ,"Que choisir - 2025-03");
    t(&dp, "Auto Plus du 21.02.2025" ,"Auto Plus - 2025-02-21");
    t(&dp, "QC_pratique_143_Mars_2025" ,"Que Choisir Pratique 143 - 2025-03");
    t(&dp, "JV113_fevrier2025" ,"JV113 fevrier2025");
    t(&dp, "Destination Portugal - Mars-Mai 2025" ,"Destination Portugal - 2025-03..05");
    t(&dp, "Charlie Hebdo - 12 Fevrier 2025" ,"Charlie Hebdo - 2025-02-12");
    t(&dp, "Direction Espagne - 03-04-05.2025" ,"Direction Espagne - 2025-03..05");
    t(&dp, "Direction Italie - 03-04-05.2025" ,"Direction Italie - 2025-03..05");
    t(&dp, "Ça M'Intéresse Histoire - Mars-Avril 2025" ,"Ça m'intéresse Histoire - 2025-03..04");
    t(&dp, "Geo France - HS - 02-03.2025" ,"Géo - HS - 2025-02..03");
    t(&dp, "Secrets d’Histoire - 03-04.2025" ,"Secrets d'Histoire - 2025-03..04");
    t(&dp, "National Geographic - 02.2025" ,"National Geographic - 2025-02");
    t(&dp, "Geo France - 02.2025" ,"Géo - 2025-02");
    t(&dp, "Total Jeux Vidéo - 01-02-03.2025" ,"Total Jeux Vidéo - 2025-01..03");
    t(&dp, "Epsiloon - 01.2025" ,"Epsiloon - 2025-01");
    t(&dp, "BBC Science Focus 2024 №414 New Year" ,"BBC Science Focus n°414 - 2024-01");
    t(&dp, "BBC Science Focus 2024 №412 November" ,"BBC Science Focus n°412 - 2024-11");
    t(&dp, "Ca m'intéresse - 02.2025" ,"Ca m'intéresse - 2025-02");
    t(&dp, "Jeux Vidéo Magazine - 02.2025" ,"Jeux Vidéo Magazine - 2025-02");
    t(&dp, "BBC Wildlife 2024 №01 January 2025" ,"BBC Wildlife n°01 - 2024-01 - 2025");
    t(&dp, "L’Auto-Journal - Le guide - 01-02-03.2025" ,"Le guide de l'Auto-Journal - 2025-01..03");
    t(&dp, "JV Le Mag - n112 - Décembre 2024" ,"JV Le Mag - n°112 - 2024-12");
    t(&dp, "Geo Histoire - 01-02.2025" ,"Géo Histoire - 2025-01..02");
    t(&dp, "Moto Revue - 02.2025" ,"Moto Revue - 2025-02");
    t(&dp, "Détours en France - 02-03.2025" ,"Détours en France - 2025-02..03");
    t(&dp, "Charlie_Hebdo_-_15_Janvier_2025" ,"Charlie Hebdo - 2025-01-15");
    t(&dp, "Le Monde - Histoire & Civilisations - 02.2025" ,"Histoire & Civilisations - 2025-02");
    t(&dp, "Le.journal.de.Mickey.N.3786-87.8.janvier.2025.FRENCH.PDF-Notag" ,"Le journal de Mickey n°3786-87 - 2025-01-08");
    t(&dp, "L’Auto-Journal du 09.01.2025" ,"L'Auto-Journal - 2025-01-09");
    t(&dp, "Les Collections de L’Histoire - Janvier-Mars 2025" ,"Les collections de L'Histoire - 2025-01..03");
    t(&dp, "L’Informaticien - Décembre 2024 - Janvier 2025" ,"L'Informaticien - 2024-12..2025-01");
    t(&dp, "Super.picsou.geant.N.246.Janvier.Fevrier.2025.FRENCH.PDF-Notag" ,"Super Picsou Géant n°246 - 2025-01..02");
    t(&dp, "Charlie Hebdo du 07.01.2025" ,"Charlie Hebdo - 2025-01-07");
    t(&dp, "Auto Plus du 03.01.2025" ,"Auto Plus - 2025-01-03");
    t(&dp, "L’Auto-Journal du 26.12.2024" ,"L'Auto-Journal - 2024-12-26");
    t(&dp, "Auto Plus - Guide de L’Acheteur - 01-02-03.2025" ,"Auto Plus Guide de l'acheteur - 2025-01..03");
    t(&dp, "L'Histoire - 01.2025" ,"L'Histoire - 2025-01");
    t(&dp, "Auto Plus du 20.12.2024" ,"Auto Plus - 2024-12-20");
    t(&dp, "Science & Vie - Hors-Série Astronomie - 01.2025" ,"Science & Vie - HS Astronomie - 2025-01");
    t(&dp, "Les cahiers de science et vie - 01-02.2025" ,"Les cahiers de Science & Vie - 2025-01..02");
    t(&dp, "Jeux Vidéo Magazine - 01.2025" ,"Jeux Vidéo Magazine - 2025-01");
    t(&dp, "L'Essentiel de l'Auto - 01-02-03.2025" ,"L'essentiel de l'Auto - 2025-01..03");
    t(&dp, "T3 France - 12.2024 01.2025" ,"T3 - 2024-12..2025-01");
    t(&dp, "Auto Plus du 29.11.2024" ,"Auto Plus - 2024-11-29");
    t(&dp, "L'histoire - N.526 - 12.2024" ,"L'histoire - n°526 - 2024-12");
    t(&dp, "Auto Plus du 22.11.2024" ,"Auto Plus - 2024-11-22");
    t(&dp, "Science & Vie - 12.2024" ,"Science & Vie - 2024-12");
    t(&dp, "Moto Journal - 12.2024" ,"Moto Journal - 2024-12");
    t(&dp, "L'essentiel de la science - 12.2024 01-02.2025" ,"L'essentiel de la science - 2024-12..2025-02");
    t(&dp, "Québec Science Décembre 2024" ,"Québec Science - 2024-12");
    t(&dp, "Moto Magazine - 12.2024" ,"Moto Magazine - 2024-12");
    t(&dp, "Moto Revue - 12.2024" ,"Moto Revue - 2024-12");
    t(&dp, "Réponses Photo - Décembre 2024" ,"Réponses Photo - 2024-12");
    t(&dp, "Canard PC - Avril 2024" ,"Canard PC - 2024-04");
    t(&dp, "Canard PC - Mai 2024" ,"Canard PC - 2024-05");
    t(&dp, "BBC Science Focus 2024 №411 October" ,"BBC Science Focus n°411 - 2024-10");
    t(&dp, "Super.picsou.geant.N.244.Septembre.Octobre.2024.francais.PDF-Notag" ,"Super Picsou Géant n°244 - 2024-09..10");
    t(&dp, "Québec Science Octobre-Novembre 2024" ,"Québec Science - 2024-10..11");
    t(&dp, "Le Canard Enchaîné du 02 Octobre 2024" ,"Le canard enchainé - 2024-10-02");
    t(&dp, "Terre Sauvage - Août 2024" ,"Terre Sauvage - 2024-08");
    t(&dp, "Le Monde - Histoire & Civilisations - 06.2024" ,"Histoire & Civilisations - 2024-06");
    t(&dp, "Le_Monde_Histoire___Civilisations_-_Juillet-Ao_t_2024" ,"Histoire & Civilisations - 2024-07..08");
    t(&dp, "L’Auto Journal du 16.05.2024" ,"L'Auto-Journal - 2024-05-16");
    t(&dp, "L'AUTO JOURNAL Le Guide - Janvier Mars 2024" ,"Le guide de l'Auto-Journal - 2024-01..03");
    t(&dp, "TERRE SAUVAGE - Février 2024" ,"Terre Sauvage - 2024-02");
    t(&dp, "01net_-_29_Novembre_2023" ,"01net - 2023-11-29");
    t(&dp, "01NET du 02 Novembre 2023" ,"01net - 2023-11-02");
    t(&dp, "Le_Journal_de_Mickey_-_25_Octobre_2023" ,"Le Journal de Mickey - 2023-10-25");
    t(&dp, "Grands_Reportages_-_Novembre_2023" ,"Grands Reportages - 2023-11");
    t(&dp, "HISTORIA - Novembre 2023" ,"Historia - 2023-11");
    t(&dp, "Le_Journal_de_Mickey_-_11_Octobre_2023" ,"Le Journal de Mickey - 2023-10-11");
    t(&dp, "Auto_Plus_-_6_Octobre_2023" ,"Auto Plus - 2023-10-06");
    t(&dp, "L’Automobile_2023_10_fr" ,"L'Automobile - 2023-10");
    t(&dp, "Le_Journal_de_Mickey_-_27_Septembre_2023" ,"Le Journal de Mickey - 2023-09-27");
    t(&dp, "LE FIGARO HISTOIRE - Octobre Novembre 2023" ,"Le Figaro Histoire - 2023-10..11");
    t(&dp, "Historia_-_Octobre_2023" ,"Historia - 2023-10");
    t(&dp, "Trek_Magazine_-_Septembre-Octobre_2023" ,"Trek Magazine - 2023-09..10");
    t(&dp, "Micro_Pratique_-_Septembre-Octobre_2023" ,"Micro Pratique - 2023-09..10");
    t(&dp, "Le_Journal_de_Mickey_-_26_Juillet_2023" ,"Le Journal de Mickey - 2023-07-26");
    t(&dp, "Elektor_France_-_Juillet-Ao_t_2023" ,"Elektor - 2023-07..08");
    t(&dp, "Le_Journal_de_Mickey_-_19_Juillet_2023" ,"Le Journal de Mickey - 2023-07-19");
    t(&dp, "Auto_Plus_Hors-S_rie_Crossovers_Suv__29_-_Mai-Juillet_2023" ,"Auto Plus HS Crossovers Suv 29 - 2023-05..07");
    t(&dp, "Auto_Moto_France_-_Juin_2023" ,"Auto Moto - 2023-06");
    t(&dp, "Science___Vie_Junior_-_Juin_2023" ,"Science Vie Junior - 2023-06");
    t(&dp, "_a_M_Int_resse_Questions___R_ponses_-_Avril-Juin_2023" ,"Ça m'intéresse Questions Réponses - 2023-04..06");
    t(&dp, "Secrets_d_Histoire_-_Juin-Ao_t_2023" ,"Secrets d'Histoire - 2023-06..08");
    t(&dp, "2023-03-01_Grands_Reportages" ,"Grands Reportages - 2023-03-01");
    t(&dp, "Elektor_France_-_Mars-Avril_2023" ,"Elektor - 2023-03..04");
    t(&dp, "L_Auto-Journal_4x4_-_Avril-Juin_2023" ,"L'Auto-Journal 4x4 - 2023-04..06");
    t(&dp, "L_Auto-Journal_-_9_Mars_2023" ,"L'Auto-Journal - 2023-03-09");
    t(&dp, "Auto_Magazine_-_Mars-Mai_2023" ,"Auto Magazine - 2023-03..05");
    t(&dp, "Comp_tence_Mac_-_Janvier-Mars_2023" ,"Compétence Mac - 2023-01..03");
    t(&dp, "Comp_tence_Photo_-_Mars-Avril_2023" ,"Comp tence Photo - 2023-03..04");
    t(&dp, "Auto_Plus_-_3_Mars_2023" ,"Auto Plus - 2023-03-03");
    t(&dp, "2023-03-01 Auto Moto" ,"Auto Moto - 2023-03-01");
    t(&dp, "Stop_Arnaques_-_Mars-Mai_2023" ,"Stop Arnaques - 2023-03..05");
    t(&dp, "Que_Choisir_Pratique_-_Mars_2023" ,"Que Choisir Pratique - 2023-03");
    t(&dp, "Que_Choisir_Sant__-_Mars_2023" ,"Que Choisir Santé - 2023-03");
    t(&dp, "Geo_France_-_Mars_2023" ,"Géo - 2023-03");
    t(&dp, "_a_M_Int_resse_-_Mars_2023" ,"Ça m'intéresse - 2023-03");
    t(&dp, "Montagnes_Magazine_-_Mars_2023" ,"Montagnes Magazine - 2023-03");
    t(&dp, "Alpes_Magazine_-_Mars-Avril_2023" ,"Alpes Magazine - 2023-03..04");
    t(&dp, "National_Geographic_Hors-S_rie_-_F_vrier-Mars_2023" ,"National Geographic HS - 2023-02..03");
    t(&dp, "Le_Journal_de_Spirou_-_8_F_vrier_2023" ,"Le Journal de Spirou - 2023-02-08");
    t(&dp, "2023-02-10_Echappee_Belle_Magazine" ,"Échappée Belle - 2023-02-10");
    t(&dp, "Science___Vie_Guerres___Histoire_-_F_vrier_2023" ,"Science & Vie Guerres & Histoire - 2023-02");
    t(&dp, "Les_Cahiers_de_Science___Vie_-_Mars-Avril_2023" ,"Les Cahiers de Science Vie - 2023-03..04");
    t(&dp, "Le_Journal_de_Spirou_-_1er_F_vrier_2023" ,"Le Journal de Spirou - 2023-02-01");
    t(&dp, "L_Automobile_Magazine_-_F_vrier_2023" ,"L'Automobile Magazine - 2023-02");
    t(&dp, "Secrets_d_Histoire_-_Mars-Mai_2023" ,"Secrets d'Histoire - 2023-03..05");
    t(&dp, "Charlie Hebdo du 1er Fvrier 2023" ,"Charlie Hebdo - 2023-02-01");
    t(&dp, "4x4_Magazine_France_-_F_vrier-Avril_2023" ,"4x4 Magazine - 2023-02..04");
    t(&dp, "Astronomy 2024 Volume 53 №02 February" ,"Astronomy - Volume 53 n°02 - 2024-02");
}