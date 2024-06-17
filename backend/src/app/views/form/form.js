document.addEventListener('DOMContentLoaded', function() {
    changeDateTimeLocalFormat();
    handleSelectAndRadioElements();
    autocomplete();

    const Block = Quill.import('blots/block');
    const Container = Quill.import('blots/container');
    const Break = Quill.import('blots/break');
    const TextBlot = Quill.import('blots/text');
    const Cursor = Quill.import('blots/cursor');

    class CodeBlockContainer extends Container {
        static create(value) {
            const domNode = super.create(value) ;
            domNode.setAttribute('spellcheck', 'false');
            domNode.setAttribute('class', 'prism-code ql-code-block-container ');
            return domNode;
        }
    }
    
    class CodeBlock extends Block {
        static TAB = '  ';
        static register() {
          Quill.register(CodeBlockContainer);
        }
    }

    CodeBlockContainer.blotName = 'code-block-container';
    CodeBlockContainer.tagName = 'pre';
    CodeBlockContainer.allowedChildren = [CodeBlock];

    CodeBlock.blotName = 'code-block';
    CodeBlock.className = 'ql-code-block';
    CodeBlock.tagName = 'DIV';
    CodeBlock.allowedChildren = [TextBlot, Break, Cursor];
    CodeBlock.requiredContainer = CodeBlockContainer;
      
    Quill.register(CodeBlock);

    var editorElements = document.querySelectorAll('.editor');
    editorElements.forEach(function(editorElement) {
        var editorId = editorElement.id;
        var textarea = document.querySelector(`textarea[data-editor-id="${editorId}"]`);
        textarea.querySelectorAll('h1, h2, h3, h4, h5, h6').forEach(function(titleElement) {
            var slug = createSlug(titleElement.textContent);
            titleElement.id = slug;
        });
        var initialContent = textarea ? textarea.value : '';
        var quill = new Quill(editorElement, {
            theme: 'snow',
            modules: {
                syntax: true,
                toolbar: [
                    ['bold', 'italic', 'underline', 'strike'],
                    ['blockquote', 'code-block'],
                    [{ 'list': 'ordered'}, { 'list': 'bullet' }],
                    [{ 'script': 'sub'}, { 'script': 'super' }],
                    [{ 'indent': '-1'}, { 'indent': '+1' }],
                    [{ 'direction': 'rtl' }],
                    [{ 'size': ['small', false, 'large', 'huge'] }],
                    [{ 'header': [1, 2, 3, 4, 5, 6, false] }],
                    [{ 'color': [] }, { 'background': [] }],
                    [{ 'font': [] }],
                    [{ 'align': [] }],
                    ['clean'],
                ],
            }
        });
        if (initialContent !== '') {
            quill.root.innerHTML = initialContent;
        }
        quill.on('text-change', function(delta, oldDelta, source) {
            var root = quill.root
            root.querySelectorAll('h1, h2, h3, h4, h5, h6').forEach(function(titleElement) {
                var slug = createSlug(titleElement.textContent);
                titleElement.id = slug;
            });
            if (source === 'user') {
                textarea.value = quill.root.innerHTML;
            }
        });
    });
});

function autocomplete() {
    const autocompleteFields = document.querySelectorAll('.autocomplete');
    const form = document.querySelector('form');
    
    const autocompleteSelectedItems = document.querySelectorAll('.autocomplete-selected-items');
    autocompleteSelectedItems.forEach(function(element) {
        element.innerHTML = '';
    });
    autocompleteFields.forEach(field => {
        field.addEventListener('input', function() {
            const id = this.getAttribute('id');
            const multiSelectSuggestionsList = form.querySelector(`#${id}List`);
            
            multiSelectSuggestionsList.innerHTML = '';
            multiSelectSuggestionsList.style.display = "block";
            if (this.value.length >= 3) {
                multiSelectSuggestionsList.style.display = "block";
                fetchData(`${this.getAttribute('data-url')}/${this.value}`, function(data) {
                    Object.keys(data).forEach(function(key) {
                        const item = document.createElement('li');
                        item.textContent = data[key][field.getAttribute('data-label')];
                        item.setAttribute('id', data[key][field.getAttribute('data-id')]);
                        item.classList.add('p-2', 'hover:bg-gray-100', 'cursor-pointer');
                        multiSelectSuggestionsList.appendChild(item);
                    });
                });
            } else {
                multiSelectSuggestionsList.style.display = "none";
            }
            
        });
        
    });
    const multiSelectSuggestionsList = form.querySelector(`.autocomplete-list`);
    multiSelectSuggestionsList.addEventListener('click', function(event) {
        if (event.target.tagName.toLowerCase()!== 'li') return;
        const parentElementDataId = event.target.parentElement.getAttribute('data-id');
        const selectedItemContainer = form.querySelector(`#${parentElementDataId}Selected`);
        const selectedId = event.target.getAttribute('id');
        const hiddenInput = document.createElement('input');

        const removeBtn = document.createElement('button');
        removeBtn.className = `
            remove-from-list
            inline-flex 
            items-center 
            gap-x-1.5 
            rounded-md 
            bg-indigo-600 
            px-3 
            py-2 
            text-sm 
            font-semibold 
            text-white 
            shadow-sm 
            hover:bg-indigo-500 
            focus-visible:outline 
            focus-visible:outline-2 
            focus-visible:outline-offset-2 
            focus-visible:outline-indigo-600
            `;
        removeBtn.textContent = event.target.textContent;
        removeBtn.appendChild(crossSvg());

        selectedItemContainer.appendChild(removeBtn)
        // const selectedId = event.target.dataset.id;
        // const selectedContainer = form.querySelector(`#${id}Selected`);
        // const existingText = textField.value;
        // const newText = `${existingText}${event.target.textContent}, `;
        // textField.value = newText.trim();

        // const hiddenInput = document.createElement('input');
        // hiddenInput.type = 'hidden';
        // hiddenInput.name = `${id}_selected`;
        // hiddenInput.value = selectedId;
        // form.appendChild(hiddenInput);

        // const removeButton = document.createElement('button');
        // removeButton.textContent = 'X';
        // removeButton.classList.add('remove-button');
        // textField.parentNode.insertBefore(removeButton, textField.nextSibling);

        // removeButton.addEventListener('click', function() {
        //     const valueArray = textField.value.split(', ');
        //     const newValue = valueArray.filter(text => text!== event.target.textContent).join(', ');
        //     textField.value = newValue;
        //     hiddenInput.remove();
        //     removeButton.remove(); 
        // });
    });

    const removeFromList = form.querySelector(`.remove-from-list`);
    removeFromList.addEventListener('click', function() {

    });
}

function crossSvg() {
    let svgIcon = document.createElementNS("http://www.w3.org/2000/svg", "svg");
    svgIcon.setAttribute("viewBox", "0 0 24 24");
    svgIcon.setAttribute("width", "24");
    svgIcon.setAttribute("height", "24");
    let path = document.createElementNS("http://www.w3.org/2000/svg", "path");
    path.setAttribute("d", "M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm0 18c-4.41 0-8-3.59-8-8s3.59-8 8-8 8 3.59 8 8-3.59 8-8 8zm-1-13h2v6h-2z");
    svgIcon.appendChild(path);
    return svgIcon;
}

function changeDateTimeLocalFormat() {
    const form = document.querySelector('form');
    const datetimeFields = form.querySelectorAll('input[type="datetime-local"]');
    datetimeFields.forEach(function(field) {
        var datetime_field = document.querySelector(`input[data-datetime="${field.id}"]`);
        field.value = datetime_field.value;
    });
    form.addEventListener('submit', function(event) {
        event.preventDefault();
        form.querySelectorAll('h1, h2, h3, h4, h5, h6').forEach(function(titleElement) {
            var slug = createSlug(titleElement.textContent);
            titleElement.setAttribute('id', slug);
        });
        datetimeFields.forEach(function(field) {
            var datetime_field = document.querySelector(`input[data-datetime="${field.id}"]`);
            let date = new Date(field.value);
            datetime_field.value = date.toISOString().slice(0, 19);
        });
        form.submit();
    });
}

function handleSelectAndRadioElements() {
    var selectElements = document.querySelectorAll('.select');
    var radioElements = document.querySelectorAll('.radio');
    selectElements.forEach(function(selectElement) {
        fetchData(selectElement.getAttribute('data-url'), function(data) {
            populateSelectOptions(selectElement, data);
        });
    });
    radioElements.forEach(function(radioElement) {
        fetchData(radioElement.getAttribute('data-url'), function(data) {
            populateRadioOptions(radioElement, data);
        });
    });
}

function fetchData(url, processData) {
    fetch(url, {
        method: 'GET', 
        headers: {
            'Content-Type': 'application/json' 
        }
    })
    .then(function(response) {
        if (!response.ok) {
            throw new Error('Fetch options error');
        }
        return response.json();
    })
    .then(processData)
    .catch(function(error) {
        console.error('Fetch options error:', error);
    });
}

function populateSelectOptions(selectElement, data) {
    selectElement.innerHTML = '';
    Object.keys(data).forEach(function(key) {
        var option = document.createElement('option');
        option.value = data[key][selectElement.getAttribute('data-id')];
        option.textContent = data[key][selectElement.getAttribute('data-label')];
        selectElement.appendChild(option);
    });
    var selectedValue = selectElement.getAttribute('data-selected');
    if (selectedValue) {
        selectElement.value = selectedValue;
    }
}

function populateRadioOptions(radioElement, data) {
    radioElement.innerHTML = '';
    Object.keys(data).forEach(function(key) {
        var name = radioElement.getAttribute('data-name');
        var value = data[key][radioElement.getAttribute('data-id')];
        var labelText = data[key][radioElement.getAttribute('data-label')];
        var selected = radioElement.getAttribute('data-selected');
        var radioHtml = createRadioInput(name, value, labelText, selected);
        radioElement.innerHTML += radioHtml;
    });
}

function createRadioInput(name, value, labelText, selected) {
    return `
        <div class="sm:col-span-4">
            <input id="${name}" name="${name}" value="${value}" ${selected == value ? 'checked' : ''} type="radio"
                class="h-4 w-4 border-gray-300 text-indigo-600 focus:ring-indigo-600">
            <label for="${name}"
                class="block text-sm font-medium leading-6 text-gray-900">${labelText}</label>
        </div>
    `;
}

function createSlug(text) {
    return text.toLowerCase().replace(/[^a-z0-9]+/g, '-').replace(/(^-|-$)/g, '');
}
