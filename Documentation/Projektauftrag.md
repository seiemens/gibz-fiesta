# GIBZ FIEŚTA

## Projekt Erläuterung

Wir haben den Auftrag bekommen, ein Tool zu entwickeln, mit dem man verschiedene Kompetenzen für den Unterricht verwalten kann. Dieses Tool hat bestimmte Anforderungen zu erfüllen. Diese sind unten aufgelistet!

## Anforderungen

### Funktional

- rollen verwaltung
	- spezielle rechte für lehrer
		- kompetenzen erstellen
		- kompetenzen löschen
		- resourcen für kompetenzen hinzufügen
		- etc
	- versch. berufe
- kompetenzen
	- bereiche e.g. *html seite erstellen*
	- level e.g. *w3 validator keine fehler - 3 punkte aka position* `|o|o|x|o|`
	- unendlich viele resourcen aka links. z.b. wie in teams eine word vorlage oder so

### Nicht Funktional

#### Vom PDF

- muss mit OOP sprache gemacht werden
- testing
	- [DOC] test konzept
	- [DOC] test fälle
	- unit tests decken **60%** ab

#### Eigene

- Dark-Mode Switch
- responsive
- block nach x falschen logins
- erweiterte log funktionen
	- online zeiten
	- ips
- (3rd Party OAUTH) authentifizierung für API requests
- DB encrypted & pws hashed
- keine unnötigen daten schicken bzgl. API und Datenbank read aka ~~select **\*** from table~~


## Technologien

Da unser Plan eine Web-Applikation zu entwickeln ist, brauchen wir passende Programmiersprachen und tools, um dieses Vorhaben umzusetzen.

Fürs Frontend haben wir uns für **Svelte** Entschieden. Es ist einfach zu verstehen und vielseitig einsetzbar.

Beim Backend legten wir uns auf **Rust** fest, da es Modern, effizient bzgl. der Resourcen und eine beliebte sprache ist, vorallem für Backend Services.

Natürlich müssen die User Daten auch irgendwo gespeichert werden. Um es uns einfacher zu machen, setzen wir hierbei nicht auf neue Technologie, sondern auf etwas, mit dem wir schon vertraut sind, nicht so wie beim Front- und Backend. Unsere Wahl ist **MongoDB**.

### Erfahrung

Für uns alle ist **Svelte** und **Rust** eigentlich neu. Ramon hat zum Teil aber schon ein kleiner Projekt mit Go und Svelte gemacht, das ist allerdings schon eine weile her. Timo hat sich auch bereits ein wenig in Rust eingearbeitet. MongoDB allerdings haben wir alle schon genutzt und verstehen wir zum grossen Teil auch recht gut.

### Tools

Um die seite zu entwickeln werden wir verschiedene Tools in anspruch nehmen. Zum einen wäre da **VSCode**, ein gratis open-source text editor. Allerdings haben wir in der Gruppe verschiedene prefärenzen, darum wird auch noch die **Jetbrains Toolbox** genutz, von der wir hauptsächlich **WebStorm** für die Frontend entwicklung und **DataGrip** für die Datenbank erstellung benutzen.
Normalerweiese kostet die JB Toolbox etwas, allerdings kann man als Schüler die Software Suite kostenlos benutzen. Von dem Angebot wird gerne gebrauch gemacht.
