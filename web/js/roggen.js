Date.prototype.ddmmyyyy = function() {
    var mm = this.getMonth() + 1;
    var dd = this.getDate();
    return [
        (dd>9? '' : '0') + dd,
        (mm>9? '' : '0') + mm,
        this.getFullYear()
    ].join('.')
}

document.addEventListener("DOMContentLoaded", function(event) {
    document.querySelectorAll('[data-published]').forEach(function(element) {
        var time = element.attributes["data-published"].value;
        if (time == "") {
            element.textContent = "Not published";
        } else {
            time = new Date(time);
            element.textContent = time.ddmmyyyy();
        }
    });
});