// SPDX-License-Identifier: MIT

#pragma once

#include <QStandardItemModel>
#include <QQmlEngine>
#include <qtmetamacros.h>


/*
  Simple model that shows JSON data in a tree or list QStandardItemModel.
  Use for quick prototyping, not for production, as performance might not scale.
*/
class JsonModel : public QStandardItemModel
{
    Q_OBJECT
    QML_ELEMENT
    Q_PROPERTY(QString jsonText READ jsonText WRITE loadJsonText NOTIFY jsonTextChanged)
public:
    explicit JsonModel(QObject *parent = nullptr);
    ~JsonModel() override;

    void loadJsonText(const QString &text);
    QString jsonText() const;

Q_SIGNALS:
    void jsonTextChanged();

private:
    void addRow(const QJsonObject &row, QStandardItem *parent);
    QString m_jsonText;
};
