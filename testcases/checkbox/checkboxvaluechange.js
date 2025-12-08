test(async () => {
    const checkbox = queryFromBody(`input[id="${checkboxId}"]`);
    assertNotUndefined(checkbox, "nincs checkbox elem");
    checkbox.checked = !checkboxdefault;
    await triggerEvent(checkbox, "change");

    const sections = queryAllFromBody(`body > div`);
    assertEquals(2, sections.length, "Nem két div van a body tag alatt közvetlenül")
    for (const elem of sections) {
        if (elem.id != defaultVisible) {
            assertEquals(true, !elem.classList.contains('hide'), `Nincs megjelenítve a szükséges táblázat és form ${elem.id}`);
        } else {
            assertEquals(true, elem.classList.contains('hide'), `Nincs elrejtve az egyik táblázat és form ${elem.id}`);
        }
    }
    return true;
}, hasDropdown);