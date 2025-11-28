const queryTableById = (id, withRow = false) => {
	const table = document.body.querySelector(`table[id="${id}"]`);
	assertNotUndefined(table, `"${id}" azonosítóval rendelkező table nincs`)
	if (!withRow) {
		return [table];
	}
	const rowNum = table.querySelectorAll('tbody tr').length;
	return [table, rowNum];
}


const queryTableByTbodyId = (id, withRow = false) => {
	const table = document.body.querySelector(`table:has(tbody[id="${id}"])`);
	assertNotUndefined(table, `"${id}" azonosítóval rendelkező tbody nincs`)
	if (!withRow) {
		return [table];
	}
	const rowNum = table.querySelectorAll('tbody tr').length;
	return [table, rowNum];
}


const queryFormById = (id) => {
	const form = document.body.querySelector(`form[id="${id}"]`);
	assertNotUndefined(form, `"${id}" azonosítóval rendelkező form nincs`)
	return form;
}


const queryInputByIdOfForm = (form, inputId) => {
	const result = form.querySelector(`input[id="${inputId}"]`)
	assertNotUndefined(result, `A ${form.id} nem tartalmaz ${inputId} azonosítójú elemet`)
	return result;
}


const setInputValueByid = (form, name, value) => {
	queryInputByIdOfForm(form, name).value = value;
}

const getInputValueByid = (form, name) => {
	return queryInputByIdOfForm(form, name).value;
}


const queryFromBody = (query) => {
	const result = document.body.querySelector(query);
	return result
}


const queryAllFromBody = (query) => {
	return document.body.querySelectorAll(query)
}

const getErrorFieldContentByInputId = (form, inputId) => {
	const errorfield = form.querySelector(`input[id="${inputId}"]+.error`);
	assertNotUndefined(errorfield, `${inputId} bevitelő mező mellett nincs error megjelenítésére szolgáló elem`)
	return errorfield.innerText;
}

const mappingCellIndex = {
	"0": "elso",
	"1": "masodik",
	"2": "harmadik",
	"3": "negyedik",
	"4": "otodik",
}

const getLastRowFromTableByTbodyId = (tbodyid) => {
	const [table] = queryTableByTbodyId(tbodyid)
	const lastrow = table.querySelector('tbody tr:last-child');
	const cells = lastrow.querySelectorAll('td');
	const result = [];
	for (const [key, cell] of cells.entries()) {
		result.push({
			id: mappingCellIndex[key],
			content: cell.innerText,
			rowspan: cell.rowSpan,
			colspan: cell.colSpan
		})
	}
	return result;
}

const getLastTwoRowFromTableByTbodyId = (tbodyid) => {
	const [table] = queryTableByTbodyId(tbodyid)
	const beforeLastRow = table.querySelector('tbody tr:nth-last-child(2)');
	const lastrow = table.querySelector('tbody tr:last-child');
	const cellsOfBeforeLast = beforeLastRow.querySelectorAll('td');
	const cells = lastrow.querySelectorAll('td');
	const mergedArray = [...Array.from(cellsOfBeforeLast), ...Array.from(cells)];
	const result = [];
	for (const [key, cell] of mergedArray.entries()) {
		result.push({
			id: mappingCellIndex[key],
			content: cell.innerText,
			rowspan: cell.rowSpan,
			colspan: cell.colSpan
		})
	}
	return result;
}