function data() {
  return {
      data: [],
      load() {
        fetch('/api/data')
          .then(res => res.json())
          .then(data => {
            console.log(data);
            this.data = data;
          });
      }
    };
}

function getColor(i, total) {
  if (i == 0) return "background: #EA3943;";
  if (i < total) return "background: #DED70D;";
  if (i == total) return "background: #16c784;";
}
