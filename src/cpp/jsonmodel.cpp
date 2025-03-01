// SPDX-License-Identifier: MIT

#include "jsonmodel.h"

#include <QJsonDocument>
#include <QJsonArray>
#include <QJsonObject>
#include <QDebug>
#include <QScopeGuard>
#include <qnamespace.h>
#include <qtcoreexports.h>

JsonModel::JsonModel(QObject *parent)
    : QStandardItemModel(parent)
{
}

JsonModel::~JsonModel()
{
}

void JsonModel::setJsonText(const QString &text)
{
    if (text == m_jsonText)
        return;

    qDebug() << Q_FUNC_INFO << text;

    m_jsonText = text;

    auto guard = qScopeGuard([this] {
        Q_EMIT jsonTextChanged();
        Q_EMIT countChanged();
    });

    QJsonDocument doc = QJsonDocument::fromJson(text.toUtf8());

    clear();
    auto root = doc.object();
    if (root.isEmpty())
        return;

    m_headerTitles.clear();
    const auto headersJson = root.value("headers").toArray();
    m_headerTitles.reserve(headersJson.size());
    for (const auto &header : headersJson) {
        m_headerTitles.append(header.toString());
    }

    auto rows = root.value("children").toArray();
    if (rows.isEmpty())
        return;

    beginResetModel();

    for (auto row : rows) {
        if (!row.isObject()) {
            qDebug() << Q_FUNC_INFO << "Row is not object";
            continue;
        }

        addRow(row.toObject(), invisibleRootItem());
    }

    endResetModel();
}

void JsonModel::addRow(const QJsonObject &row, QStandardItem *parent)
{
    auto item = new QStandardItem(row.value("name").toString());
    parent->appendRow(item);

    const auto children = row.value("children").toArray();
    for (const auto &child : children) {
        if (!child.isObject()) {
            qDebug() << Q_FUNC_INFO << "Child is not object";
            continue;
        }

        addRow(child.toObject(), item);
    }
}

QString JsonModel::jsonText() const
{
    return m_jsonText;
}

int JsonModel::count() const
{
    return rowCount();
}

QVariant JsonModel::headerData(int section, Qt::Orientation orientation, int role) const
{
    if (role != Qt::DisplayRole || section < 0 || section >= m_headerTitles.size())
        return {};

    return m_headerTitles[section];
}
