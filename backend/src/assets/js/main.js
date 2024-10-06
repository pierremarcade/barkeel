import { beforeSubmit, handleSelectAndRadioElements, handleFileElements, handleAutocompleteElements, handleCheckboxElements } from './form.js';
import { init } from './quill.js';

function sortTable(columnName, order) {
    const table = document.getElementById('myTable');
    const tbody = table.tBodies[0];

    // Récupération des lignes
    let rows = Array.from(tbody.rows);

    // Tri des lignes
    rows.sort((rowA, rowB) => {
        const cellA = rowA.cells.find(cell => cell.dataset.sort === columnName);
        const cellB = rowB.cells.find(cell => cell.dataset.sort === columnName);

        if (!cellA || !cellB) return 0;

        const valueA = cellA.textContent.trim();
        const valueB = cellB.textContent.trim();

        if (valueA === valueB) return 0;

        const isAscending = order === 'asc';
        return isAscending ?
            (valueA > valueB ? 1 : -1) :
            (valueA < valueB ? 1 : -1);
    });

    // Remplacement des lignes triées
    tbody.innerHTML = '';
    rows.forEach(row => tbody.appendChild(row));
}


function handleColumnClick(event) {
    const th = event.target.closest('th[data-sort]');
    if (!th) return;

    const columnName = th.dataset.sort;
    const currentOrder = th.dataset.order || 'asc';

    let newOrder = 'desc';
    if (currentOrder === 'desc') {
        newOrder = 'none';
    }

    th.dataset.order = newOrder;
    updateUrlParameter(columnName, newOrder);

    if (newOrder !== 'none') {
        sortTable(columnName, newOrder);
    }
}

function updateUrlParameter(key, value) {
    const urlParams = new URLSearchParams(window.location.search);
    const params = [];

    if (urlParams.has('order')) {
        const orders = urlParams.get('order').split(',');
        orders.forEach(order => {
            const [col, dir] = order.split('_');
            if (col === key) {
                if (value !== 'none') {
                    params.push(`${key}_${value}`);
                }
            } else {
                params.push(order);
            }
        });
    } else if (value !== 'none') {
        params.push(`${key}_${value}`);
    }

    urlParams.set('order', params.join(','));
    window.history.replaceState({}, '', `${window.location.pathname}?${urlParams.toString()}`);
}


document.addEventListener('DOMContentLoaded', function () {
    handleCheckboxElements();
    handleSelectAndRadioElements();
    handleFileElements();
    handleAutocompleteElements();
    init();
    beforeSubmit();

    document.querySelectorAll('th[data-sort]').forEach(th => {
        th.addEventListener('click', handleColumnClick);
    });
});
