---
title: "Choosing thresholds for predictive models"
description: "Choosing a risk-score threshold requires balancing sensitivity, positive predictive value, alert burden, and the consequences of missed events."
date: "2026-01-05"
---

Many predictive models produce a score that ranks observations by their estimated risk of a future event.
The score alone does not determine what happens next.
When the score is used to trigger an alert or response, a threshold must define which scores count as positive predictions.
Choosing that threshold requires balancing *finding as many true events as possible* with *avoiding unnecessary alerts*.

## Threshold selection is not probability calibration.

Threshold selection is sometimes loosely described as calibrating a model, but the terms answer different questions.
When predictions represent probabilities, *probability calibration* asks whether predicted probabilities agree with observed event frequencies.
*Threshold selection* asks which scores should trigger action.
A model can have well-calibrated probabilities and still require a threshold that reflects the intended use.
This article focuses on threshold selection.

## Sensitivity and PPV describe different parts of the trade-off.

**Sensitivity** (also called recall): *Of all events that occurred, what percentage were correctly predicted?*

**Positive predictive value** (PPV; also called precision): *Of all positive predictions, what percentage corresponded to events?*

Lowering the threshold classifies more observations as positive.
This usually catches more events and increases sensitivity, but it also generates more alerts and may decrease PPV.
Raising the threshold usually produces fewer, more reliable alerts and increases PPV, but it may decrease sensitivity by missing more events.

The figure below shows the usual direction of these relationships rather than exact or guaranteed curves.
Their shapes depend on the model, population, outcome, and prediction horizon.

[![A qualitative graph showing sensitivity generally decreasing and positive predictive value generally increasing as the alert threshold rises. Lower thresholds correspond to more alerts and sensitivity-prioritized decisions; higher thresholds correspond to fewer alerts and PPV-prioritized decisions.](threshold-tradeoff.svg)](threshold-tradeoff.svg)

[Open the full-size figure.](threshold-tradeoff.svg)

## PPV depends on how common the event is.

PPV is not a fixed property of a model or threshold.
It also depends on the prevalence, or base rate, of the event in the population being evaluated.
When an event is rare, even a useful model may generate many alerts that do not correspond to events.
The same model and threshold can therefore have different PPV in populations with different event rates.

Sensitivity and PPV should be evaluated in the intended population and over the intended prediction horizon.
They should also be reassessed when the population, event rate, or operational setting changes.

## The appropriate threshold depends on consequences and capacity.

There is no universally correct threshold.
The choice should reflect the severity of the outcome, the consequences of missed events and unnecessary alerts, available resources, and downstream workflows.

| Approach | Relative threshold | Sensitivity | PPV | Alert volume | Often appropriate when |
|---|---|---|---|---|---|
| Sensitivity-prioritized | Lower | Higher | Lower | Higher | Missing an event has severe consequences and the response to an alert is relatively low risk or low cost |
| Balanced | Application-dependent | A compromise | A compromise | Moderate | Priorities are mixed, workflows are adaptable, or a pilot is being used to learn about operational effects |
| PPV-prioritized | Higher | Lower | Higher | Lower | Responses are costly, capacity is limited, or unnecessary alerts could cause harm or alert fatigue |

These approaches describe relative priorities, not universal settings.
A balanced threshold does not necessarily give sensitivity and PPV equal weight.
It should reflect the consequences of each kind of error in the intended application.
