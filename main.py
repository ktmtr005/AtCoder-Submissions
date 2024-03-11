#!/usr/bin/env python
# coding: utf-8
import push_submissions
from fetch_submission import fetch_submission

new_submissions = fetch_submission.fetchSubmissions()
push_submissions.pushSubmission(new_submissions)
