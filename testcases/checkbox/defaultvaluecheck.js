test(() => {
    const checkbox = queryFromBody(`input[id="${checkboxId}"]`);
    assertNotUndefined(checkbox, "nincs checkbox elem");
    return assertEquals(checkboxdefault, checkbox.checked, "Checkbox alapértelmezett állapota nem megfelelő");
})