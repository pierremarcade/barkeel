import { beforeSubmit, handleSelectAndRadioElements, handleFileElements, handleAutocompleteElements } from './form.js';
import { init } from './quill.js';

document.addEventListener('DOMContentLoaded', function() {
    handleSelectAndRadioElements();
    handleFileElements();
    handleAutocompleteElements();
    init();
    beforeSubmit();
});
