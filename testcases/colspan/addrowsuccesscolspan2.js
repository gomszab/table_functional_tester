 test(async ()=> {

    const form = queryFormById('htmlform')
	assertNotUndefined(form)

    const [_table, originalRows ]= queryTableByTbodyId('htmltbody', true);

    setInputValueByid(form, 'elso','Test item')
    setInputValueByid(form, 'masodik', 'test item 2');
	setInputValueByid(form, 'harmadik','Test item 3')

    await triggerSubmit(form)

    const [__table, newRows ]= queryTableByTbodyId('htmltbody', true);

    assertEquals(originalRows+1, newRows, "Nem adott hozza sort");
    const lastRow = getLastRowFromTableByTbodyId('htmltbody');
    assertEquals(3, lastRow.length);
    assertEquals("Test item", lastRow[0].content);
    assertEquals("test item 2", lastRow[1].content);
    assertEquals("Test item 3", lastRow[2].content);
    
    for(let i =0; i< 2; i++){
        assertEquals(1, lastRow[i].colspan);
        assertEquals(1, lastRow[i].rowspan);
    }
    assertEquals(2, lastRow[2].colspan);
    return assertEquals(1, lastRow[2].rowspan);
 })