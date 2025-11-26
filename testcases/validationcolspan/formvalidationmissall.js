test(async () => {
	const form = queryFormById(colform)
	assertNotUndefined(form)


	const [_table, originalRows ]= queryTableByTbodyId(coltablebody, true);


	await triggerSubmit(form)
	
	const [_, newRows ]= queryTableByTbodyId(coltablebody, true);
    const errorMessageelso = getErrorFieldContentByInputId(form, 'elso');
    const errorMessageMasodik = getErrorFieldContentByInputId(form, 'masodik');
    const errorMessageHarmadik = getErrorFieldContentByInputId(form, 'harmadik');
    assertNotEmptyString(errorMessageelso, `1. beviteli mező erorüzenete nem jelenik meg`);
    assertNotEmptyString(errorMessageMasodik, `2. beviteli mező erorüzenete nem jelenik meg`);
    assertNotEmptyString(errorMessageHarmadik, `3. beviteli mező erorüzenete nem jelenik meg`);
	return assertEquals(originalRows, newRows, "Sort fűzött hozzá a táblázathoz a validáció ellenére");
})