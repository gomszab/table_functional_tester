const assertNotUndefined = (element, text = 'Not defined') => {
    if (!element) {
        throw new Error(text);
    }
}


const assertNotEmptyString = (element, text = 'empty String') => {
    if (!element) {
        throw new Error(text);
    }
}

const assertEmptyString = (element, text = 'empty String') => {
    if (element) {
        throw new Error(text);
    }
}


const assertEquals = (element1, element2, text = 'Not equals') => {
    if (element1 === element2) {
        return true;
    } else {
        throw new Error(`${text}: ${element1} <> ${element2}`);
    }
}

const assertNotEquals = (element1, element2, text = 'Equals') => {
    if (element1 === element2) {
        return true;
    } else {
        throw new Error(`${text}: ${element1} <> ${element2}`);
    }
}