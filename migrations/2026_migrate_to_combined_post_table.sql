-- REGEXP_REPLACE(comment, '<b>(.*?)</b>', $$'''\1'''$$, 'g')
-- REGEXP_REPLACE(comment, '<i>(.*?)</i>', $$''\1''$$, 'g')
-- REGEXP_REPLACE(comment, '<span class="quote">&gt;(.*?)</span>', '>\1', 'g')
-- REGEXP_REPLACE(comment, '<a class="ref".*?>&gt;&gt;(.*?)</a>', '>>\1', 'g')
-- REGEXP_REPLACE(comment, '<span class="spoiler".*?>(.*?)</span>', '[spoiler]\1[/spoiler]', 'g')
-- REGEXP_REPLACE(comment, '<s>(.*?)</s>', '~~\1~~', 'g')
-- REGEXP_REPLACE(comment, '<span class="redText".*?>(.*?)</span>', '==\1==', 'g')
-- REGEXP_REPLACE(comment, '<a href="(.*?)".*?>(.*?)</a>', ' [\1] [\2] ', 'g')
-- REGEXP_REPLACE(comment, '<figure class="fig-comment"><img src="(.*?)".*?><figcaption>(.*?)</figcaption></figure>', ' [\1] ', 'g')

INSERT INTO Post (
    id, board, subject, comment,
    user_id, user_trip, user_country, user_name,
    birth_time, change_time, bump_time,
    reply_count, is_redacted
) SELECT
    id::BIGINT, board, subject, REGEXP_REPLACE(
      REGEXP_REPLACE(
	REGEXP_REPLACE(
	  REGEXP_REPLACE(
	    REGEXP_REPLACE(
	      REGEXP_REPLACE(
		REGEXP_REPLACE(
		  REGEXP_REPLACE(
		    REGEXP_REPLACE(comment,
		      '<b>(.*?)</b>',
		      $$'''\1'''$$,
		      'g'),
		    '<i>(.*?)</i>',
		    $$''\1''$$,
		    'g'),
		  '<span class="quote">&gt;(.*?)</span>',
		  '>\1',
		  'g'),
		'<a class="ref".*?>&gt;&gt;(.*?)</a>',
		'>>\1',
		'g'),
	      '<span class="spoiler".*?>(.*?)</span>',
	      '[spoiler]\1[/spoiler]',
	      'g'),
	    '<s>(.*?)</s>',
	    '~~\1~~',
	    'g'),
	  '<span class="redText".*?>(.*?)</span>',
	  '==\1==',
	  'g'),
	'<a href="(.*?)".*?>(.*?)</a>',
	' [\1] [\2] ',
	'g'),
      '<figure class="fig-comment"><img src="(.*?)".*?><figcaption>(.*?)</figcaption></figure>',
      ' [\1] ',
      'g'),
    uid, trip, COALESCE(country, 'ZZ'), COALESCE(uname, 'বানন'),
    ctime, mtime, mtime,
    reply_count, redacted
FROM Thread
ON CONFLICT (id) DO NOTHING;

INSERT INTO Post (
    id, thread_id, comment,
    user_id, user_trip, user_country, user_name,
    birth_time
) SELECT
    id::BIGINT, tid, REGEXP_REPLACE(
      REGEXP_REPLACE(
	REGEXP_REPLACE(
	  REGEXP_REPLACE(
	    REGEXP_REPLACE(
	      REGEXP_REPLACE(
		REGEXP_REPLACE(
		  REGEXP_REPLACE(
		    REGEXP_REPLACE(comment,
		      '<b>(.*?)</b>',
		      $$'''\1'''$$,
		      'g'),
		    '<i>(.*?)</i>',
		    $$''\1''$$,
		    'g'),
		  '<span class="quote">&gt;(.*?)</span>',
		  '>\1',
		  'g'),
		'<a class="ref".*?>&gt;&gt;(.*?)</a>',
		'>>\1',
		'g'),
	      '<span class="spoiler".*?>(.*?)</span>',
	      '[spoiler]\1[/spoiler]',
	      'g'),
	    '<s>(.*?)</s>',
	    '~~\1~~',
	    'g'),
	  '<span class="redText".*?>(.*?)</span>',
	  '==\1==',
	  'g'),
	'<a href="(.*?)".*?>(.*?)</a>',
	' [\1] [\2] ',
	'g'),
      '<figure class="fig-comment"><img src="(.*?)".*?><figcaption>(.*?)</figcaption></figure>',
      ' [\1] ',
      'g'),
    uid, trip, COALESCE(country, 'ZZ'), COALESCE(uname, 'বানন'),
    ctime
FROM Reply
ON CONFLICT (id) DO NOTHING;
