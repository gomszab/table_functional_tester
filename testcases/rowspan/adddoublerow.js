test(async () => {
	const form = queryFormById('jsform')
	assertNotUndefined(form)


	const [_table, originalRows ]= queryTableByTbodyId('jstbody', true);
    

	setInputValueByid(form, 'elso','Test item 1')
	setInputValueByid(form, 'masodik','Test item 2')
	setInputValueByid(form, 'harmadik','Test item 3')
    setInputValueByid(form, 'negyedik','Test item 4')
    setInputValueByid(form, 'otodik','Test item 5')



	await triggerSubmit(form)
	
	const [_, newRows ]= queryTableByTbodyId('jstbody', true);
	assertEquals(originalRows+2, newRows, "Nem fuzott hozza sort");
    const lastRow = getLastTwoRowFromTableByTbodyId('jstbody');
    assertEquals("Test item 1", lastRow[0].content);
    assertEquals("Test item 2", lastRow[1].content);
    assertEquals("Test item 3", lastRow[2].content);
    assertEquals("Test item 4", lastRow[3].content);
    assertEquals("Test item 5", lastRow[4].content);
    assertEquals(1, lastRow[0].colspan);
    assertEquals(2, lastRow[0].rowspan);
    for(let i=1; i< lastRow.length; i++){
        assertEquals(1, lastRow[i].colspan);
        assertEquals(1, lastRow[i].rowspan);
    }
    return true;
})