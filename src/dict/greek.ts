import type { Dict } from '../types';

export const greekLetters: Dict = [
	[/а/, 'α'],
	[/б/, 'μπ'],
	[/в/, 'β'],
	[/г/, 'γ'],
	[/ґ/, 'γκ'],
	[/д/, 'δ'],
	[/е/, 'η'],
	[/ё/, 'υο'],
	[/ж/, 'ζ̌'],
	[/з/, 'ζ'],
	[/і/, 'ι'],
	[/й/, 'υ'],
	[/к/, 'κ'],
	[/л/, 'λ'],
	[/мп/, 'μ‘π'],
	[/м/, 'μ'],
	[/н/, 'ν'],
	[/о/, 'ο'],
	[/п/, 'π'],
	[/р/, 'ρ'],
	[/с/, 'σ'],
	[/сь/, 'ς'],
	[/т/, 'τ'],
	[/у/, 'ϋ'],
	[/ў/, 'ΰ'],
	[/ф/, 'φ'],
	[/х/, 'χ'],
	[/ц/, 'τσ'],
	[/ць/, 'τς'],
	[/ч/, 'τσ̌'],
	[/ш/, 'σ̌'],
	[/ы/, 'ί'],
	[/э/, 'ε'],
	[/ю/, 'υϋ'],
	[/я/, 'υα'],
	[/ь/, '’'],
];

export const greekLettersUpperCase: Dict = [
	[/ Б(?= *\p{Ll})/u, ' Μπ'],
	[/ Ґ(?= *\p{Ll})/u, ' Γκ'],
	[/ Ё(?= *\p{Ll})/u, ' Υο'],
	[/ Ю(?= *\p{Ll})/u, ' Υϋ'],
	[/ Я(?= *\p{Ll})/u, ' Υα'],
	[/ Ч(?= *\p{Ll})/u, ' Τσ̌'],
	[/Б/, 'ΜΠ'],
	[/Ґ/, 'ΓΚ'],
	[/Ё/, 'ΥΟ'],
	[/Ю/, 'ΥΫ'],
	[/Я/, 'ΥΑ'],
	[/Ч/, 'ΤΣ̌'],

	[/А/, 'Α'],
	[/В/, 'Β'],
	[/Г/, 'Γ'],
	[/Д/, 'Δ'],
	[/Е/, 'Η'],
	[/Ж/, 'Ζ̌'],
	[/З/, 'Ζ'],
	[/І/, 'Ι'],
	[/Й/, 'Υ'],
	[/К/, 'Κ'],
	[/Л/, 'Λ'],
	[/МП/, 'Μ‘Π'],
	[/Мп/, 'Μ‘π'],
	[/М/, 'Μ'],
	[/Н/, 'Ν'],
	[/О/, 'Ο'],
	[/П/, 'Π'],
	[/Р/, 'Ρ'],
	[/С/, 'Σ'],
	[/С[Ьь]/, 'ς'],
	[/Т/, 'Τ'],
	[/У/, 'Ϋ'],
	[/Ў/, 'Ϋ́'],
	[/Ф/, 'Φ'],
	[/Х/, 'Χ'],
	[/Ц/, 'Τσ'],
	[/Ц[Ьь]/, 'Τς'],
	[/Ш/, 'Σ̌'],
	[/Ы/, 'Ί'],
	[/Э/, 'Ε'],
	[/Ь/, '’'],
];

export const thWords: Dict = [
	[/тэі(?=зм|ст)/, 'θει'],
	[/ агат/, ' αγαθ'],
	[/ анатэм/, ' αναθεμ'],
	[/антрапа/, 'ανθραπα'],
	[/ апатэ(?=оз|аты|м)/, ' απαθε'],
	[/ арта(?=графі|д[ао]кс|эпі)/, ' αρθα'],
	[/ арытмэт/, ' αρίθμετ'],
	[/ атэн/, ' αθεν'],
	[/ калітэ/, ' καλιθε'],
	[/ катэд(?=\(?а?р)/, ' καθεδ'],
	[/лябірынт/, 'λυαμπιρίνθ'],
	[/лягарытм/, 'λυαγαρίθμ'],
	[/ мар[фт]ы /, ' μαρθί '],
	[/ марфе /, ' μαρθη '],
	[/ матэматы/, ' μαθεματί'],
	[/ міт(?=[ауы]|оляг| )/, ' μιθ'],
	[/ мэт(?=[ао]д)/, ' μεθ'],
	[/пат(?=ал[ёя]гі|оляг|ычн|ыя|ы[ійю] |[ауы] |а[ўм] |амі | )/, 'παθ'],
	[/ пітагор/, ' πιθαγορ'],
	[/ ры[тф]м/, ' ρίθμ'],
	[/сынтэ[зт]/, 'σίνθετ'],
	[/ томас/, ' θομασ'],
	[/ тромб/, ' θρομμπ'],
	[/ трон/, ' θρον'],
	[/трамбоз/, 'θραμμποζ'],
	[/ тэадос/, ' θεαδοσ'],
	[/ тэарэм/, ' θεαρεμ'],
	[/ тэат(?=ар |р)/, ' θεατ'],
	[/ тэм(?=[аеуы])/, ' θεμ'],
	[/ тэ(?=[ао]р|салёнік|сал)/, ' θε'],
	[/тэ(?=с\S*алёнік|салё\)нік|\S*зіс)/, 'θε'],
	[/тэрма/, 'θερμα'],
	[/ эт(?=ы[кч]|эр)/, ' εθ'],
];
