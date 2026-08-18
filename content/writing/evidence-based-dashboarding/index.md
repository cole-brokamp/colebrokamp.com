---
title: "Evidence-based dashboarding"
description: "A dashboard can be based on data without being evidence-based; dashboard design should begin with intended decisions, testable requirements, and evidence that the interface works."
date: "2026-08-18"
---

Dashboards are routinely based on data. Far fewer are supported by evidence that they work.

A dashboard should be a tool for helping people interpret information, make decisions, and take action—not a collection of charts. Yet dashboards are often judged by whether they contain the requested data, function technically, or look polished. Those things matter, but they do not tell us whether a dashboard is effective. A technically correct dashboard can still overwhelm its users, obscure uncertainty, encourage false confidence, or direct attention toward the wrong problem.

*Evidence-based dashboarding* describes the disciplined design, evaluation, and stewardship of [human-data interfaces](https://www.sciencedirect.com/science/article/pii/S1071581919301193) that help specific people make specific decisions.

## Evidence-based dashboarding requires more than data.

The term is intentionally provocative. It echoes evidence-based medicine, where the qualifier can sound almost unnecessary. The need for the term reflects the gap between that expectation and actual practice.

> A dashboard can be based on data without being evidence-based.

In its classic formulation, [evidence-based medicine](https://www.bmj.com/content/312/7023/71) is not the mechanical application of research findings. It integrates the best available external evidence with professional expertise and the values and circumstances of the patient. Evidence informs judgment in context; it does not replace it.

Evidence-based dashboarding should work similarly. It brings together:

- the best available research from data visualization, human-computer interaction, and the relevant substantive domain;
- the expertise of domain professionals, designers, developers, and data stewards;
- the goals, values, abilities, and lived experience of intended users and people affected by their decisions; and
- empirical evidence about how the interface performs in its actual setting.

The analogy also changes the status of the dashboard itself. A [dashboard is an intervention](https://link.springer.com/article/10.1186/s13012-025-01430-x). It should have an **indication**: a defined decision or action it is meant to support. It should have a plausible **mechanism**: an explanation of how particular information and interactions are expected to improve understanding or judgment. Its intended **benefits** should be stated in advance. Potential **harms**—including [cognitive overload](https://doi.org/10.1093/jamiaopen/ooab096), [misinterpretation](https://doi.org/10.1073/pnas.2302491120), [false precision](https://doi.org/10.1073/pnas.2302491120), [inequitable access](https://doi.org/10.2196/11342), and [misplaced attention](https://doi.org/10.1093/jamiaopen/ooab096)—should also be considered. And, like any intervention, a dashboard requires monitoring after it is introduced.

## Different claims require different evidence.

Evidence is needed at several stages, and different questions require different methods.

**Prior evidence** helps identify established principles, known failure modes, and appropriate measures. **Contextual evidence** comes from observing workflows and listening to intended users and affected communities. **Evaluative evidence** tests whether people can use the resulting interface as intended and whether its use improves decisions or outcomes.

[No single method can answer every question](https://doi.org/10.2196/59828). [Interviews and co-design](https://doi.org/10.2196/28854) can reveal needs and assumptions, but they cannot establish that users interpret a display correctly. [Task-based testing](https://doi.org/10.2196/humanfactors.9569) can measure interpretation accuracy or completion time, but it may not predict sustained use. [Usage analytics](https://doi.org/10.2196/28854) can show that a dashboard was opened, but not that it helped. Outcome evaluation can examine whether decisions or actions changed, but stronger claims require stronger study designs.

Evidence-based dashboarding therefore does not mean demanding a randomized trial before publishing a chart. It means matching the evidence to the claim and being explicit about what remains unknown.

Five principles connect these forms of evidence to dashboard development and evaluation.

## 1. Define the people and decisions before the interface.

Identify the intended users, the people affected by their decisions, the questions being asked, and the setting in which the information will be used. “Stakeholders” is often too broad. The person using a dashboard, the person acting on its information, and the person affected by that action may not be the same.

## 2. Translate intended use into testable requirements.

A decision-oriented user story connects a particular person and workflow to an immediate use of information:

> **User story:** As a [specific user], when [situation or workflow], I need to [answer a question, make a decision, or take an action] using [information], so that I can [immediate benefit].

The corresponding success criterion states what evidence would demonstrate that the interface is effective:

> **Success criterion:** The interface is effective if [user] can [observable task] with [specified accuracy, time, confidence, or other threshold].

For example:

> **User story:** As a public-health planner preparing the weekly testing-site schedule, I need to compare neighborhood disease burden with geographic access to existing sites so that I can identify underserved areas for additional testing.
>
> **Success criterion:** The interface is effective if planners can accurately identify and explain the highest-priority areas within five minutes.

This pairing matters. A user story without a success criterion expresses an aspiration. A success criterion without a user story risks measuring something disconnected from an actual decision.

## 3. Design around decisions and workflows.

The information, visual hierarchy, interactions, explanatory content, accessibility, privacy protections, and update frequency should follow from the intended use—not from what happens to be available in the database. Data quality, provenance, timeliness, uncertainty, and representativeness are interface concerns because they affect what conclusions a user can reasonably draw.

The goal is not to show everything. It is to provide the minimum information necessary to support the intended interpretation, decision, or action without hiding material uncertainty or alternatives.

## 4. Evaluate effectiveness throughout development.

Evaluation should begin before the dashboard is finished. Interviews, observation, focus groups, and co-design can expose incorrect assumptions and support formative refinement. Prototypes can be tested for comprehension, task completion, decision quality, cognitive workload, accessibility, and usability.

After implementation, evaluation should shift toward [use in context](https://doi.org/10.2196/28854): adoption, reach, workflow fit, unintended consequences, decision changes, and domain-specific outcomes. [Satisfaction is useful evidence, but it is not evidence of effectiveness on its own](https://www.jmir.org/2026/1/e98272).

## 5. Use evidence to iterate—and sometimes to stop.

A dashboard is not finished when it is launched. Changes in users, data, workflows, policies, or decisions should prompt renewed evaluation. Evidence may justify adding something, but it may also justify simplifying the interface, changing its purpose, or retiring it altogether.

Iteration should not mean accumulating features. It should mean repeatedly asking whether the dashboard still achieves its intended benefit with acceptable burden and risk.

## Evidence-based dashboarding changes the starting question.

The usual dashboard project begins by asking what should appear on the dashboard. Evidence-based dashboarding instead requires the development team to identify:

- the specific decision the interface is intended to improve;
- how the information and interactions are expected to improve that decision;
- the evidence that would demonstrate that the interface works; and
- the evidence that would justify changing or discontinuing it.
