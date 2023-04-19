use rand_derive2::RandGen;
use strum_macros::{EnumString, Display};

#[derive(Debug, Display, RandGen, EnumString)]
pub enum FirstNameEnglish {
    Ryan, Mike, Jalen, Jaden,
    Seth, Stephen, Cam, James,
    Tobias, Henry, Paul, Anthony,
    Spencer, Royce, Joe, Nick,
    Joel, Michael, Mikal, Tyrese,
    Dyson, Samson, Webster, Talon,
    Morley, Chad, Elliot, Doug,
    Orrell, Cosmo,
}

#[derive(Debug, Display, RandGen, EnumString)]
pub enum LastNameEnglish {
    Johnson, Williams, Brown, Jones,
    Garcia, Miller, Davis, Rodriguez,
    Martinez, Hernandez, Lopez, Gonzales,
    Wilson, Anderson, Thomas, Taylor,
    Moore, Jackson, Martin, Lee,
    Indie, Howard, Shirley, Westbrook,
    Rowe, Barclay, Bannister, Hartell,
    Reed, Beckham, Warrick, Lyndon,
    Tucker, Denzil,
}    

#[derive(Debug, Display, RandGen, EnumString)]
pub enum FirstNameAfrican{
    Emmanuel, Jermaine, Jalen,
    Amari, Deshawn, Odell,
    Deon, Tevin, Devonte,
    Mekhi, Omari, Tyrese,
    Jabari, Dennis, Kamari,
    Daquan, Dandre, Ike,
    Zaire, Akeem, Rashawn,
    Demond, Amare, Keyon,
    Deshaun, Donell, Antione,
    Savion, Jelani, Niles,
    Ammon, Obie, Hermon,
    Tavon, Zyaire, Musa,
    Omarion, Kwame, Kymani,
    Treyvon, Makhi, Yoel,
    Khari, Jakari, Juelz,
    Mikal, Nile, Dwane,
    Kenyatta, Rashaun, Ramses,
    Dakari, Jovon, Najee,
    Elam, Amon, Willem,
    Remi, Maury, Devaughn,
    Tavaris, Osiris, Laquan,
    Amani, Damani, Theodis,
    Arvel, Raekwon, Jakai,
    Xavion, Kenya, Jacorey,
    Demetric, Johannes, Vernard,
    Acie, Levar, Nakia,
    Tj, Neo, Tyreese,
    Dontavious, Tyrin, Rashon,
    Ajani, Marquan, Khamari,
    Amauri, Zaylen, Kimani,
    Kejuan, Kordell, Kobi,
    Mamadou, Rapheal, Tyjuan,
    Treshawn, Marquell, Lebron,
    Okey,
}


#[derive(Debug, Display, RandGen, EnumString)]
pub enum LastNameAfrican {
    Mohamed, Ali, Ahmed,
    Ibrahim, Hassan, Diallo,
    Musa, Abdullahi, Abubakar,
    Sani, Traore, Adamu,
    Usman, Umar, Osman,
    Garba, Hussein, Kone,
    Banda, Juma, Joseph,
    Issa, Bello, Bah,
    Sow, Sawadogo, Deng,
    Ismail, Solomon, Barry,
    Phiri, Diarra, Idris,
    Keita, Abdou, Cisse,
    Saleh, Yahya, Lawal,
    Tadesse, Brown, Mayo,
    Odom, Gay, Odell,
    Coker, Godwin, Mena,
    Queen, Wessel, Elam, 
    Rodas, Niles, Lebron, 
    Ivory,
}


#[derive(Debug, Display, RandGen, EnumString)]
pub enum FirstNameSlav{
    Alex, Nikita, George, Dima,
    Daniel, Adam, Sasha, Vlad,
    Dávid, Sergey, Alexander, Ivan,
    Mateusz, Artem, Martin, Peter,
    Andrey, Pavel, Anton, Andrew,
    Nick, John, Max, Dmitry,
    Jakub, Igor, Szymon, Michael,
    Vladimir, Kamil, Bartek, Mark,
    Maxim, Egor, Alexandr, Dominik,
    Adrian, Patryk, Filip, Kirill,
    Roman, Andrei, Micha, Denis,
    Ilya, Jan, Oleg, Marcin,
    Michal, Robert, Vadim, Ruslan,
    Paul, Patrik, Dawid, Vladislav,
    Lukas, Kuba, Danil, Artur,
    Wojtek, Bogdan, Marek, Tamás,
    Maciej, Tomas, Mike, Tomek,
    Kostya, Sebastian, Balazs, Gleb,
    Alexey, Zhenya, Petr, Bálint,
    Ond, Bence, Richard, Tomá,
    Kostas, Matthew, Tasos, Kacper,
    Pawe, Dimitris, Boris, Andreas,
    Tom, Bartosz, Thanos, Karol,
    Misha, Honza, Stelios, Chris,
    László, Attila, Yaroslav, Krystian,
}


#[derive(Debug, Display, RandGen, EnumString)]
pub enum LastNameSlav{
    Barno, Ganus, Ivanov,
    Rosya, Smirnov, Volkov,
    Archaki, Bartos, Bosko,
    Hakel, Kowalski, Soroka,
    Adamovich, Aksamit, Ivanova,
    Gutnik, Shevchenko, Antic,
    Franic, Horvat, Jankovic,
    Kovanovic, Nikolic, Nikola,
    Novak, Jokic, Jovic,
    Popovik, Zoric,
}



#[derive(Debug, Display, RandGen, EnumString)]
pub enum FirstNameGerman{
    Lambart, Ekerd, Adel, Agustine,
    Kuno, Roswell, Roch, Wallache,
    Humphrey, Rainor, Alric, Walmond,
    Humberto, Spangler, Baldric, Per,
    Eberhardt, Gerhard, Erroll, Derry,
    Jurgen, Maynor, Augustine, Margit,
    Loring, Arik, Wolfrik, Berthold,
    Raimundo, Terrall, Orland, Burnard,
    Hackett, Oswald, Bernd, Audrick,
    Reinhard, Martell, Amaud, Hildbrand,
    Garry, Baldwin, Warren, Stefan,
    Erhard, Brock, Erchanhardt, Bing,
    Audwin, Alarick, Ludwig, Waller,
    Kolt, Griswald, Kulbert, Terrel,
    Berg, Maximilian, Eugen, Brand,
    Amo, Garrey, Weber, Kurt,
    Raynard, Engel, Brandeis, Gilbert,
    Waldrom, Adolfo, Hewlett, Hoh,
    Gottfried, Harvey, Amell, Rollie,
    Ann, Robbie, Otis, Fonsie,
    Wolfric, Luiginw, Arick, Waldmunt,
    Berend, Brunon, Gerrell, Bemelle,
    Berdy, Aurick, Bren, Hewett,
    Robert,
}

#[derive(Debug, Display, RandGen, EnumString)]
pub enum LastNameGerman{
    Busch,Brenneman,Grunst,Knolle,
    Hengesbach,Morgenroth,Rubner,Huppert,
    Banning,Baumstark,Sendelbach,Wehmeyer,
    Harz,Mittelstaedt,Haage,
    Huwe,Ertz,Heser,
    Ness,Koelbl,Wacker,Jack,
    Westfall,Prather,Hemauer,Lulay,
    Heinkel,Imgrund,Buseman,Wehrle,
    Benzschawel,Stauder,Nehring,Bache,
    Oltz,Patschke,Lanterman,Schiele,
    Ratz,Pollman,Rohrig,Haber,
    Groshans,Bolter,Benthin,Vorpahl,
    Zens,Burkheimer,Welk,Weichel,
    Kittner,Sickel,Plate,Loesche,
    Worm,Sebert,Merle,Specht,
    Scherz,Tretter,Trautner,
}

#[derive(Debug, Display, RandGen, EnumString)]
pub enum FirstNameFrench{
   Millard,Etie,Thomure,Mangin,
   Filiatrault,Polin,Houde,
   Violette,Cler,Cloud,Vallin,Leboeuf,
   Grimard,Blais,Villines,Pruit, Passe,
   Maze,Vincelette,Fossett,Hayes,
   Billard,Delorme,Louise,
   Roux,Gendron,Brian,
   Bazin,Plaisance,Glasson,
   Prevost,Jolicoeur,Teston,
   Palen,Dury,Hellard,Palin,
   Choplin,Mery,Dambrose,
   Moret,Perot,Hardin,
   Grate,Croteau,Grandchamp,
   Devillers,Fauteux,Giraud,
   Monfort,Prunier,Juneau,
   Junot,Metivier,Harry,
   Mabey,Marsan,Danger,
   Leclere,Lafleche,Dargis,
   Pare,Vautrin,Carron,
}
#[derive(Debug, Display, RandGen, EnumString)]
pub enum LastNameFrench{
   Ulrich,Deveral,Herve,
   Raymund,Rust,Napoleon,
   Dionte,Archard,Colyn,
   Leeroy,Yvet,Burdette,
   Guillaume,Talbot,Valentin,
   Plat,Chaney,Boone,
   Campbell,Destrie,Joseph,
   Rique,Rosiyn,Hubert,
   Garen,Thieny,Montgomery,
   Eliott,Maxime,Noel,
   Somerville,Raoul,Russel,
   Paulin,Leverett,Anselme,
   Geoff,Patric,Chandler,
   Vallois,Eriq,Rollie,
   Edmon,Prosper,
   Preruet,Delmore,Elliot,
   Royal,Rainier,Warrane,
   Basile,Brunelle,Remi,
   Paul,Cort,
   Fernand,Mantel,Delron,
   Dartagnan,Channing,Rayce,
   Arnaud,Perry,Barry,
}
