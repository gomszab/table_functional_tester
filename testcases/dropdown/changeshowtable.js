test(async () => {
    const select = queryFromBody('select[id="tableselector"]')
    assertNotUndefined(select, "nincs select elem a megadott id-val");
    
    const htmlSectionDiv = queryFromBody(`div[id="${htmlSection}"]`)
    assertNotUndefined(htmlSectionDiv, "nincs select elem a megadott id-val");
    const jsSectionDiv = queryFromBody(`div[id="${jsSection}"]`)
    assertNotUndefined(jsSectionDiv, "nincs select elem a megadott id-val");
    if(jsSectionDiv.classList.contains('hide')){
        select.value = jsSection;
        await triggerEvent(select, 'change');
        const divList = queryAllFromBody(`div[id="${htmlSection}"], div[id="${jsSection}"]`)
        for (const div of divList) {
            if (div.id === jsSection) {
                assertEquals(false, div.classList.contains('hide'), `A ${div.id} azonosítójú elem nincs megjelenítve, a select ${select.value}-ra változtatása után`);
            } else {
                assertEquals(true, div.classList.contains('hide'), `A ${div.id} azonosítójú elem nincs elrejtve, a select ${select.value}-ra változtatása után`);
            }
        }
    }

    if(htmlSectionDiv.classList.contains('hide')){
        select.value = htmlSection;
        await triggerEvent(select, 'change');
        const divList  = queryAllFromBody(`div[id="${htmlSection}"], div[id="${jsSection}"]`)
        for (const div of divList) {
            if (div.id === htmlSection) {
                assertEquals(false, div.classList.contains('hide'), `A ${div.id} azonosítójú elem nincs megjelenítve, a select ${select.value}-ra változtatása után`);
            } else {
                assertEquals(true, div.classList.contains('hide'), `A ${div.id} azonosítójú elem nincs elrejtve, a select ${select.value}-ra változtatása után`);
            }
        }
    }
    
    return true;
}, hascheckbox)