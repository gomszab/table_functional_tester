test(async () => {
    const form = queryFormById(colform)
    assertNotUndefined(form)


    const [_table, originalRows] = queryTableByTbodyId(coltablebody, true);


    setInputValueByid(form, 'elso', 'Test item')
    setInputValueByid(form, 'harmadik', 'Test item1')


    await triggerSubmit(form)

    const [_, newRows] = queryTableByTbodyId(coltablebody, true);
    const errorMessageMasodik = getErrorFieldContentByInputId(form, 'masodik');
    assertNotEmptyString(errorMessageMasodik,  "Második mezőnél nem jelenik meg hibaüzenet");
    assertEquals(originalRows, newRows, "Sort fűzött hozzá a táblázathoz a validáció ellenére");
    setInputValueByid(form, 'masodik', 'test item 2');
    await triggerSubmit(form)
    const elemList = ['elso', 'masodik', 'harmadik'];
    elemList.map(elem =>
        getErrorFieldContentByInputId(form, elem)
    ).forEach((element, index) => {
        assertEmptyString(element, `${index + 1}. beviteli mező erorüzenete nem tünt el`);
    });
    const [__, newRowsAgain] = queryTableByTbodyId(coltablebody, true);
    return assertNotEquals(originalRows + 1, newRowsAgain, "Validációs hibák javítása után nem jelent meg új sor");
})