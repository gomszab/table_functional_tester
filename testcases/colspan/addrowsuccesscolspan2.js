test(async () => {

    const form = queryFormById(colform)
    assertNotUndefined(form)

    const [_table, originalRows] = queryTableByTbodyId(coltablebody, true);

    setInputValueByid(form, 'elso', 'Test item')
    setInputValueByid(form, 'masodik', 'test item 2');
    setInputValueByid(form, 'harmadik', 'Test item 3')

    await triggerSubmit(form)

    const [__table, newRows] = queryTableByTbodyId(coltablebody, true);

    assertEquals(originalRows + 1, newRows, "Nem adott hozza sort");
    const lastRow = getLastRowFromTableByTbodyId(coltablebody);
    assertEquals(3, lastRow.length);
    assertEquals("Test item", lastRow[0].content, "A hozzáadott táblázathoz hozzáadott elem 1. oszlopának tartalma nem jó");
    assertEquals("test item 2", lastRow[1].content, "A hozzáadott táblázathoz hozzáadott elem 2. oszlopának tartalma nem jó");
    assertEquals("Test item 3", lastRow[2].content, "A hozzáadott táblázathoz hozzáadott elem 3. oszlopának tartalma nem jó");

    for (let i = 0; i < 2; i++) {
        assertEquals(1, lastRow[i].colspan, `A hozzáadott táblázathoz hozzáadott elem ${i + 1}. oszlopának colspan értéke nem jó`);
        assertEquals(1, lastRow[i].rowspan, `A hozzáadott táblázathoz hozzáadott elem ${i + 1}. oszlopának colspan értéke nem jó`);
    }
    assertEquals(2, lastRow[2].colspan, `A hozzáadott táblázathoz hozzáadott elem 3. oszlopának colspan értéke nem jó`);
    return assertEquals(1, lastRow[2].rowspan, `A hozzáadott táblázathoz hozzáadott elem 3. oszlopának rowspan értéke nem jó`);
})