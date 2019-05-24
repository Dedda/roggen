Date.prototype.ddmmyyyy = function() {
    var mm = this.getMonth() + 1;
    var dd = this.getDate();
    return [
        (dd>9? '' : '0') + dd,
        (mm>9? '' : '0') + mm,
        this.getFullYear()
    ].join('.')
}

var getCookie = function getCookie(cname) {
    var name = cname + "=";
      var decodedCookie = decodeURIComponent(document.cookie);
      var ca = decodedCookie.split(';');
      for(var i = 0; i <ca.length; i++) {
        var c = ca[i];
        while (c.charAt(0) == ' ') {
          c = c.substring(1);
        }
        if (c.indexOf(name) == 0) {
          return c.substring(name.length, c.length);
        }
      }
      return "";
};

var getLanguage = function getLanguage() {
    var lang = getCookie('selected-language');
    return lang == '' ? 'en' : lang;
};

document.addEventListener("DOMContentLoaded", function(event) {
    document.querySelectorAll('[data-published]').forEach(function(element) {
        var time = element.attributes["data-published"].value;
        if (time == "") {
            switch (getLanguage()) {
                case 'de':
                    element.textContent = "Nicht veröffentlicht";
                    break;
                default:
                    element.textContent = "Not published";
            }
        } else {
            time = new Date(time);
            element.textContent = time.ddmmyyyy();
        }
    });
    document.querySelectorAll('[data-toggle-language]').forEach(function(element) {
        element.addEventListener('click', function() {
            var lang = element.attributes['data-toggle-language'].value;
            document.cookie = "selected-language=" + lang + ";path=/";
            location.reload();
        });
    });
});