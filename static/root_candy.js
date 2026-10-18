const bengaliDigits = [ '০', '১', '২', '৩', '৪', '৫', '৬', '৭', '৮', '৯' ];

function toBengaliNumerals(num) {
  return String(num).split('').map(digit => bengaliDigits[digit] || digit).join('');
}

(function () {
    const counts = document.querySelectorAll('.reply-count');

    counts.forEach(count => {
	const count_translit = toBengaliNumerals(count.textContent);
	count.textContent = count_translit;
    })
})()
