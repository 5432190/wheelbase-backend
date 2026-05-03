const form = document.querySelector("#bridgeForm");
const resultCard = document.querySelector("#resultCard");

function createElement(tag, className, text) {
  const element = document.createElement(tag);

  if (className) {
    element.className = className;
  }

  if (text) {
    element.textContent = text;
  }

  return element;
}

function formatPoints(points) {
  return new Intl.NumberFormat("en-US").format(points);
}

function getTier(points, travelers, flexibility) {
  const pointsPerTraveler = points / travelers;

  if (travelers > 3 || flexibility === "Low") {
    return {
      name: "Concierge",
      price: "$79",
      reason: "Your route has more moving pieces, so a human booking plan is the best fit.",
    };
  }

  if (pointsPerTraveler >= 10000 || flexibility === "High") {
    return {
      name: "Bridge Plan",
      price: "$29",
      reason: "There is enough flexibility or point value to justify a reviewed bridge itinerary.",
    };
  }

  return {
    name: "Starter",
    price: "$9",
    reason: "Start with a quick scan to see whether your points create useful savings.",
  };
}

function getPattern(points, travelers, priority) {
  const pointsPerTraveler = points / travelers;

  if (pointsPerTraveler >= 15000 && priority === "Use the most Spirit points") {
    return "Lead with a Spirit award leg, then add the cheapest one-way connector on another carrier.";
  }

  if (priority === "Fewest stops") {
    return "Compare the best nonstop cash fare against one Spirit-points leg plus one connector.";
  }

  if (priority === "Best family-friendly route") {
    return "Prioritize daytime flights, longer connection buffers, and airports with easier transfers.";
  }

  return "Use Spirit points only where they beat cash value, then bridge the rest with a low-cost carrier or ground transfer.";
}

form.addEventListener("submit", (event) => {
  event.preventDefault();

  const data = new FormData(form);
  const from = data.get("from").trim();
  const to = data.get("to").trim();
  const points = Number(data.get("points"));
  const travelers = Number(data.get("travelers"));
  const flexibility = data.get("flexibility");
  const priority = data.get("priority");
  const tier = getTier(points, travelers, flexibility);
  const pointsPerTraveler = Math.floor(points / travelers);
  const mailtoBody = [
    `Route: ${from} to ${to}`,
    `Points: ${points}`,
    `Travelers: ${travelers}`,
    `Flexibility: ${flexibility}`,
    `Priority: ${priority}`,
  ].join("\n");

  resultCard.replaceChildren();
  resultCard.append(
    createElement("p", "eyebrow", "Your bridge strategy"),
    createElement("h3", null, `${from} to ${to}`),
  );

  const tierStat = createElement("div", "result-stat");
  tierStat.append(
    createElement("span", null, "Recommended tier"),
    createElement("strong", null, `${tier.name} - ${tier.price}`),
  );

  const pointStat = createElement("div", "result-stat");
  pointStat.append(
    createElement("span", null, "Points per traveler"),
    createElement("strong", null, `${formatPoints(pointsPerTraveler)} pts`),
  );

  const requestLink = createElement("a", "button button-full", "Request manual quote");
  requestLink.href = `mailto:hello@example.com?subject=${encodeURIComponent("Spirit Points Bridge Request")}&body=${encodeURIComponent(mailtoBody)}`;

  resultCard.append(
    tierStat,
    pointStat,
    createElement("p", null, tier.reason),
    createElement("p", null, getPattern(points, travelers, priority)),
    requestLink,
  );
});
