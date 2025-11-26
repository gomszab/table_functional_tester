# table_functional_tester
A table function tester alkalmazással funkcionálisan tudod tesztelni (egyelőre) a 2 táblázatos alkalmazásodat. Jelenleg 25 tesztből áll, a testesetek leírását [itt](./testcases/config.json), a tesztesetek javascriptben vannak és többnyire querySelectorral operál. Ha lefut mind a 25 teszt akkor jó eséllyel jól működik a progid. De természetesen a kódodat továbbra is át fogom nézni.

# Telepítés:
Amikor először futtatod, le fog tölteni egy headless_chrome alkalmazást (az indítófájlal egy mappába), és a tesztek futtatását ott végzi. Ha a headless_chrome mappát nem törlöd, a többi teszt már gyorsabban le fog futni.

# Használat:
Hogy le tudd futtatni a tesztet, szükséged lesz egy config.json fájlra az index.html fájl mellé.
Innen olvassa fel a configurációt.
A mintadolgozat megoldásának config fájlját [innen](./config.json) tudod letölteni, amit az index.html mellé kell másolnod.
az alkalmazás indítása: Parancssorból kell indítanod az alkalmazást (pl powershell) és beilleszted a teljes fájlelérési útját az index.html-nek amit szeretnél tesztelni.
pl.: .\table_function_tester.exe "e:\bolyai\2025_26\mintadoga\index.html"

A config jsonnal tudsz generálni configokat pár egyszerű kérdés megválaszolásával.
(TODO)