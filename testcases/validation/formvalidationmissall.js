test(async () => {
	const form = queryFormById('htmlform')
	assertNotUndefined(form)


	const [_table, originalRows ]= queryTableByTbodyId('htmltbody', true);


	await triggerSubmit(form)
	
	const [_, newRows ]= queryTableByTbodyId('htmltbody', true);
    const errorMessageelso = getErrorFieldContentByInputId(form, 'elso');
    const errorMessageMasodik = getErrorFieldContentByInputId(form, 'masodik');
    const errorMessageHarmadik = getErrorFieldContentByInputId(form, 'harmadik');
    assertNotEmptyString(errorMessageelso);
    assertNotEmptyString(errorMessageMasodik);
    assertNotEmptyString(errorMessageHarmadik);
	return assertEquals(originalRows, newRows, "Nem fuzott hozza sort");
})